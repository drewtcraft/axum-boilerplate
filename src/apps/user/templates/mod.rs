use askama::Template;

#[derive(Template)]
#[template(path = "log-in.html")]
pub struct LogInTemplate();

#[derive(Template)]
#[template(path = "sign-up.html")]
pub struct SignUpTemplate();
