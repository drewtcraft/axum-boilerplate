use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    pub body: String,
}
