use askama::Template;

#[derive(Template)]
#[template(path = "log-in.html")]
pub struct LogInTemplate();
