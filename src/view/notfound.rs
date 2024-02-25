use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};

use crate::{error::Result, view::HtmlTemplate};

#[derive(Template)]
#[template(path = "404.html")]
struct Template404 {
    title: String,
}

pub async fn handler_404() -> Result<impl IntoResponse> {
    let template = Template404 {
        title: "Page not found".to_string(),
    };

    Ok(HtmlTemplate(template, Some(StatusCode::NOT_FOUND)))
}
