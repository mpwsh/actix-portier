use crate::routes::prelude::*;

#[derive(Deserialize)]
pub struct BasicForm {
    pub name: String,
    pub loading: Option<bool>,
}

pub async fn dashboard(
    hb: web::Data<Handlebars<'_>>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user = req
        .extensions()
        .get::<UserId>()
        .expect("UserId should be present after middleware check")
        .to_string();

    let is_htmx = req.headers().contains_key("HX-Request");

    let template = if is_htmx { "home" } else { "dashboard" };

    let data = json!({
        "title": "Dashboard",
        "user": user,
    });
    let body = hb.render(template, &data).unwrap();

    Ok(HttpResponse::Ok().body(body))
}
