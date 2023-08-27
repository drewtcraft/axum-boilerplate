use sailfish::TemplateOnce;
use crate::error::{Result, Error};

#[derive(TemplateOnce)]
#[template(path = "base.stpl")]
pub struct BaseTemplate {
    pub content: String,
}

impl BaseTemplate {
    pub fn new_render(content: String) -> Result<String> {
        Self { content }
            .render_once()
            .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
pub struct ErrorTemplate {
    pub error_message: String,
    pub status_code: String,
}

impl ErrorTemplate {
    pub fn new(is_htmx: bool, error_message: String, status_code: String) -> String {
        let rendered = Self { error_message, status_code }
            .render_once()
            .unwrap_or("Error template failure.".to_string());

        if is_htmx {
            rendered
        } else {
            BaseTemplate::new_render(rendered)
                .unwrap_or("Error template failure.".to_string())
        }
    }
}
