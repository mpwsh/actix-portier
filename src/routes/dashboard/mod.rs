use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

use crate::{
    session_state::TypedSession,
    utils::{e500, see_other},
};

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
