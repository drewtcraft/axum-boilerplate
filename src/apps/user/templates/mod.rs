use askama::Template;

#[derive(Template)]
#[template(path = "log-in.html")]
pub struct LogInTemplate();

#[derive(Template)]
#[template(path = "sign-up.html")]
pub struct SignUpTemplate {
    pub email: String,
}

#[derive(Template)]
#[template(path = "send-invite.html")]
pub struct SendInviteTemplate();
