use std::net::TcpListener;

use actix_files as fs;
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{self, Key},
    dev::Server,
    web::{self, resource},
    App, HttpServer, Result,
};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use actix_web_lab::middleware::from_fn;
use handlebars::Handlebars;
use portier::Client;
use secrecy::{ExposeSecret, Secret};
use tracing_actix_web::TracingLogger;

use crate::{
    authentication::reject_anonymous_users,
    config::Settings,
    errors::error_handlers,
    routes::{claim, dashboard, health, login, login_form, logout, whoami},
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            config.application.base_url,
            config.application.hmac_secret,
            config.redis_uri,
            config.portier_uri,
            config.application.session_ttl,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

pub async fn run(
    listener: TcpListener,
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
    portier_uri: Secret<String>,
    session_ttl: i64,
) -> Result<Server, anyhow::Error> {
    let claim_path = "/claim";
    let redirect_uri = format!("{base_url}{claim_path}").parse().unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".hbs", "./templates")?;

    let client = Client::builder(redirect_uri)
        .broker(portier_uri.expose_secret().parse().unwrap())
        .build()
        .unwrap();
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(cookie::time::Duration::hours(session_ttl)),
                    )
                    .build(),
            )
            .wrap(error_handlers())
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(handlebars.clone()))
            .route("/", web::get().to(login_form))
            .route("/health", web::get().to(health))
            .service(
                resource("/login")
                    .route(web::get().to(login_form))
                    .route(web::post().to(login)),
            )
            .service(
                resource("/dashboard")
                    .wrap(from_fn(reject_anonymous_users))
                    .route(web::get().to(dashboard)),
            )
            .service(resource(claim_path).route(web::post().to(claim)))
            .service(
                resource("/logout")
                    .route(web::get().to(logout))
                    .route(web::post().to(logout)),
            )
            .service(resource("/whoami").route(web::get().to(whoami)))
            .service(fs::Files::new("/static/", "./static/"))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
