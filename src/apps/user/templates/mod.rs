use crate::error::{Error, Result};
use crate::traits::ToPlainText;
use askama::Template;

// --------------
// USER TEMPLATES
// --------------
#[derive(Template)]
#[template(path = "log-in.html")]
pub struct LogInTemplate<'a> {
    pub input_error: Option<&'a str>,
}

impl<'a> LogInTemplate<'a> {
    pub fn new_render() -> Result<String> {
        Self { input_error: None }
            .render()
            .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(input_error: Option<&'a str>) -> Result<String> {
        Self { input_error }
            .render()
            .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(Template)]
#[template(path = "sign-up.html")]
pub struct SignUpTemplate<'a> {
    pub attached_email: &'a str,
    pub username: Option<&'a str>,
    pub username_input_error: Option<&'a str>,
}

impl<'a> SignUpTemplate<'a> {
    pub fn new_render(attached_email: &'a str) -> Result<String> {
        Self {
            attached_email,
            username: None,
            username_input_error: None,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(
        attached_email: &'a str,
        username: Option<&'a str>,
        username_input_error: Option<&'a str>,
    ) -> Result<String> {
        Self {
            attached_email,
            username,
            username_input_error,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(Template)]
#[template(path = "send-invite.html")]
pub struct SendInviteTemplate<'a> {
    input_error: Option<&'a str>,
}

impl<'a> SendInviteTemplate<'a> {
    pub fn new_render() -> Result<String> {
        Self { input_error: None }
            .render()
            .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(input_error: Option<&'a str>) -> Result<String> {
        Self { input_error }
            .render()
            .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(Template)]
#[template(path = "log-out.html")]
pub struct LogOutTemplate();

impl LogOutTemplate {
    pub fn new_render() -> Result<String> {
        Self().render().map_err(|_| Error::TemplateRenderingFailure)
    }
}

// ---------------
// EMAIL TEMPLATES
// ---------------

#[derive(Template)]
#[template(path = "email_invite.html")]
pub struct EmailInviteTemplate {
    pub acceptance_url: String,
}

impl EmailInviteTemplate {
    pub fn new(acceptance_url: String) -> Self {
        EmailInviteTemplate { acceptance_url }
    }
}

impl ToPlainText for EmailInviteTemplate {
    fn to_plain_text(&self) -> String {
        format!("Sign up url: {}", &self.acceptance_url)
    }
}

#[derive(Template)]
#[template(path = "email_log-in.html")]
pub struct EmailLogInTemplate {
    pub log_in_url: String,
}

impl EmailLogInTemplate {
    pub fn new(log_in_url: String) -> Self {
        Self { log_in_url }
    }
}

impl ToPlainText for EmailLogInTemplate {
    fn to_plain_text(&self) -> String {
        format!("Log in url: {}", &self.log_in_url)
    }
}

// ---------------
// ADMIN TEMPLATES
// ---------------

pub type FieldError<'a> = Option<&'a str>;

pub struct UserListUser<'a> {
    pub id: i64,
    pub username: Option<&'a str>,
    pub email: &'a str,
    pub active: bool,
    pub role: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Template)]
#[template(path = "admin_user-list.html")]
pub struct AdminUserListTemplate<'a> {
    pub users_list: Option<Vec<UserListUser<'a>>>,

    pub user_id_input: Option<&'a str>,
    pub user_id_input_error: FieldError<'a>,

    pub username_input: Option<&'a str>,
    pub username_input_error: FieldError<'a>,

    pub active_input: Option<&'a str>,

    pub email_input: Option<&'a str>,
    pub email_input_error: FieldError<'a>,

    pub role_input: Option<&'a str>,

    pub sort_by: Option<&'a str>,  // username, email
    pub sort_dir: Option<&'a str>, // ASC, DESC
}

impl<'a> AdminUserListTemplate<'a> {
    pub fn new_render(
        users_list: Option<Vec<UserListUser<'a>>>,
        user_id_input: Option<&'a str>,
        username_input: Option<&'a str>,
        active_input: Option<&'a str>,
        email_input: Option<&'a str>,
        role_input: Option<&'a str>,
        sort_by: Option<&'a str>,
        sort_dir: Option<&'a str>,
    ) -> Result<String> {
        Self {
            users_list,
            user_id_input,
            user_id_input_error: None,
            username_input,
            username_input_error: None,
            active_input,
            email_input,
            email_input_error: None,
            role_input,
            sort_by,
            sort_dir,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(
        user_id_input: Option<&'a str>,
        user_id_input_error: FieldError<'a>,
        username_input: Option<&'a str>,
        username_input_error: FieldError<'a>,
        active_input: Option<&'a str>,
        email_input: Option<&'a str>,
        email_input_error: FieldError<'a>,
        role_input: Option<&'a str>,
        sort_by: Option<&'a str>,
        sort_dir: Option<&'a str>,
    ) -> Result<String> {
        Self {
            users_list: None,
            user_id_input,
            user_id_input_error,
            username_input,
            username_input_error,
            active_input,
            email_input,
            email_input_error,
            role_input,
            sort_by,
            sort_dir,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(Template)]
#[template(path = "admin_user-edit.html")]
pub struct AdminUserEditTemplate<'a> {
    pub user_id: Option<&'a str>,

    pub username: Option<&'a str>,
    pub username_input_error: FieldError<'a>,

    pub email: Option<&'a str>,
    pub email_input_error: FieldError<'a>,

    pub active: Option<bool>,

    pub role: Option<&'a str>,

    pub success_message: Option<&'a str>,
    pub submit_url: &'a str,
}

impl<'a> AdminUserEditTemplate<'a> {
    pub fn new_render_error(
        user_id: Option<&'a str>,
        username: Option<&'a str>,
        username_input_error: FieldError<'a>,
        email: Option<&'a str>,
        email_input_error: FieldError<'a>,
        active: Option<bool>,
        role: Option<&'a str>,
        submit_url: &'a str,
    ) -> Result<String> {
        Self {
            user_id,
            username,
            username_input_error,
            email,
            email_input_error,
            active,
            role,
            submit_url,
            success_message: None,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_existing(
        user_id: &'a String,
        username: Option<&'a str>,
        email: &'a str,
        active: bool,
        role: &'a str,
        submit_url: &'a str,
        success_message: Option<&'a str>,
    ) -> Result<String> {
        Self {
            user_id: Some(user_id.as_str()),
            username,
            username_input_error: None,
            email: Some(email),
            email_input_error: None,
            active: Some(active),
            role: Some(role),
            submit_url,
            success_message,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_blank(submit_url: &'a str) -> Result<String> {
        Self {
            user_id: None,
            username: None,
            username_input_error: None,
            email: None,
            email_input_error: None,
            active: None,
            role: None,
            submit_url,
            success_message: None,
        }
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)
    }
}
