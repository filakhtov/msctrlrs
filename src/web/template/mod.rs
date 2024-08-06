use crate::web::{self, session};
use actix_web::{error, http::header};

#[derive(serde::Serialize)]
pub struct Content<C: serde::Serialize> {
    app_version: &'static str,
    content: C,
    flash_messages: Vec<session::FlashMessage>,
}

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<C: serde::Serialize> Content<C> {
    pub fn new(flash_messages: session::FlashMessages, content: C) -> Self {
        Self {
            content,
            app_version: APP_VERSION,
            flash_messages: flash_messages.take(),
        }
    }
}

pub fn render_template<N: AsRef<str>, C: serde::Serialize>(
    templates: &handlebars::Handlebars,
    name: N,
    data: &Content<C>,
) -> Result<String, error::InternalError<&'static str>> {
    match templates.render(name.as_ref(), data) {
        Ok(content) => Ok(content),
        Err(err) => {
            eprintln!("Failed to render Handlebar template: {err}");

            Err(web::internal_server_error())
        }
    }
}

pub fn render_response<N: AsRef<str>, C: serde::Serialize>(
    templates: &handlebars::Handlebars,
    name: N,
    data: &Content<C>,
) -> Result<actix_web::HttpResponse, error::InternalError<&'static str>> {
    render_template(templates, name, data).map(|content| {
        actix_web::HttpResponse::Ok()
            .content_type(header::ContentType::html())
            .body(content)
    })
}
