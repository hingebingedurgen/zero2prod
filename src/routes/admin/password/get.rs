use actix_web::{HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    dbg!(&msg_html);
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(include_str!("get.html"), msg_html = msg_html)))
}
