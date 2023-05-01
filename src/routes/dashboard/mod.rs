use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::web;
use actix_web::HttpResponse;
use handlebars::Handlebars;
use serde_json::json;

pub async fn dashboard(
    hb: web::Data<Handlebars<'_>>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let user = if let Some(email) = session.get_user().map_err(e500)? {
        email
    } else {
        return Ok(see_other("/login"));
    };
    let data = json!({
        "user": user,
    });
    let body = hb.render("dashboard", &data).unwrap();

    Ok(HttpResponse::Ok().body(body))
}
