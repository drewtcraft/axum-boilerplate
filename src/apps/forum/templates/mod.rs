use crate::error::{Error, Result};
use crate::templates::BaseTemplate;
use crate::traits::ToPlainText;
use askama::Template;
use sailfish::TemplateOnce;

use super::serializers::{NewThread, NewThreadErrors};

// ----------------
// THREAD TEMPLATES
// ----------------
#[derive(TemplateOnce)]
#[template(path = "new-thread.stpl")]
pub struct NewThreadTemplate {
    pub params: NewThread,
    pub params_errors: NewThreadErrors,
}

impl NewThreadTemplate {
    pub fn new_render(is_htmx: bool) -> Result<String> {
        let rendered = Self {
            params: NewThread::default(),
            params_errors: NewThreadErrors::default(),
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)?;

        if is_htmx {
            Ok(rendered)
        } else {
            BaseTemplate::new_render(rendered)
        }
    }

    pub fn new_render_error(
        is_htmx: bool,
        params: NewThread,
        params_errors: NewThreadErrors,
    ) -> Result<String> {
        let rendered = Self {
            params,
            params_errors,
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)?;

        if is_htmx {
            Ok(rendered)
        } else {
            BaseTemplate::new_render(rendered)
        }
    }
}

pub struct ThreadPost<'a> {
    title: &'a str,
    content: &'a str,
    username: &'a str,
    created_at: &'a str,
    updated_at: &'a str,
}

#[derive(Template)]
#[template(path = "get-thread.html")]
pub struct ThreadTemplate<'a> {
    title: &'a str,
    username: &'a str,
    created_at: &'a str,
    updated_at: &'a str,
    posts: Vec<ThreadPost<'a>>,
    total_pages: i64,
    page: i64,
    limit: i64,
    offset: i64,
    sort_dir: &'a str,
}
