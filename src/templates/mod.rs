use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate();

#[derive(Template)]
#[template(path = "base-htmx.html")]
pub struct BaseHtmxTemplate();
