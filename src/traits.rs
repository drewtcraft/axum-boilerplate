use sailfish::TemplateOnce;

use crate::error::{Error, Result};

pub trait ToPlainText: TemplateOnce {
    fn to_plain_text(&self) -> String;
    fn render_html_and_plain_text(self) -> Result<(String, String)> {
        let plain_text = self.to_plain_text();
        let rendered_email_template = self.render_once().map_err(|_| Error::TemplateRenderingFailure)?;

        Ok((rendered_email_template, plain_text))
    }
}

pub trait ParamValidator<T> {
    fn validate(&self) -> (bool, T);
}

pub trait ParamValidationError {
    fn new_empty() -> Self;
}

