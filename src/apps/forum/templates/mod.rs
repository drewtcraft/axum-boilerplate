use crate::error::{Error, Result};
use crate::traits::ToPlainText;
use askama::Template;

// ----------------
// THREAD TEMPLATES
// ----------------
#[derive(Template)]
#[template(path = "new-thread.html")]
pub struct NewThreadTemplate<'a> {
    pub title: Option<&'a str>,
    pub title_input_error: Option<&'a str>,
    pub content: Option<&'a str>,
    pub content_input_error: Option<&'a str>,
}

impl<'a> NewThreadTemplate<'a> {
    pub fn new_render() -> Result<String> {
        Self {
            title: None,
            title_input_error: None,
            content: None,
            content_input_error: None,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(
        title: Option<&'a str>,
        title_input_error: Option<&'a str>,
        content: Option<&'a str>,
        content_input_error: Option<&'a str>,
    ) -> Result<String> {
        Self {
            title,
            title_input_error,
            content,
            content_input_error,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
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
