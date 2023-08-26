use super::serializers::{UserListParams, UserListParamsErrors};
use crate::error::{Error, Result};
use crate::traits::{ParamValidationError, ToPlainText};
use askama::Template;
use sailfish::TemplateOnce;

// --------------
// USER TEMPLATES
// --------------
#[derive(TemplateOnce)]
#[template(path = "log-in.stpl")]
pub struct LogInTemplate {
    pub input_error: Option<String>,
}

impl LogInTemplate {
    pub fn new_render() -> Result<String> {
        Self { input_error: None }
            .render_once()
            .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(TemplateOnce)]
#[template(path = "sign-up.stpl")]
pub struct SignUpTemplate {
    pub attached_email: String,
    pub username: Option<String>,
    pub username_input_error: Option<String>,
}

impl SignUpTemplate {
    pub fn new_render(attached_email: String) -> Result<String> {
        Self {
            attached_email,
            username: None,
            username_input_error: None,
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(
        attached_email: String,
        username: Option<String>,
        username_input_error: Option<String>,
    ) -> Result<String> {
        Self {
            attached_email,
            username,
            username_input_error,
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(TemplateOnce)]
#[template(path = "send-invite.stpl")]
pub struct SendInviteTemplate {
    pub input_error: Option<String>,
}

impl SendInviteTemplate {
    pub fn new_render() -> Result<String> {
        Self { input_error: None }
            .render_once()
            .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(input_error: Option<String>) -> Result<String> {
        Self { input_error }
            .render_once()
            .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(TemplateOnce)]
#[template(path = "log-out.stpl")]
pub struct LogOutTemplate;

impl LogOutTemplate {
    pub fn new_render() -> Result<String> {
        Self.render_once()
            .map_err(|_| Error::TemplateRenderingFailure)
    }
}

// ---------------
// EMAIL TEMPLATES
// ---------------

#[derive(TemplateOnce)]
#[template(path = "email_invite.stpl")]
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

#[derive(TemplateOnce)]
#[template(path = "email_log-in.stpl")]
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

pub struct UserListUser {
    pub id: i64,
    pub username: Option<String>,
    pub email: String,
    pub active: bool,
    pub user_role_id: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(TemplateOnce)]
#[template(path = "admin_user-list.stpl")]
pub struct AdminUserListTemplate {
    pub users_list: Option<Vec<UserListUser>>,
    pub user_roles: Vec<(i32, String)>,
    pub query_params: UserListParams,
    pub query_params_errors: UserListParamsErrors,
}

impl AdminUserListTemplate {
    pub fn new_render(
        users_list: Option<Vec<UserListUser>>,
        user_roles: Vec<(i32, String)>,
        query_params: UserListParams,
    ) -> Result<String> {
        Self {
            users_list,
            user_roles,
            query_params,
            query_params_errors: UserListParamsErrors::new_empty(),
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_error(
        user_roles: Vec<(i32, String)>,
        query_params: UserListParams,
        query_params_errors: UserListParamsErrors,
    ) -> Result<String> {
        Self {
            users_list: None,
            user_roles,
            query_params,
            query_params_errors,
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)
    }
}

// #[derive(Template)]
// #[template(path = "admin_user-edit.html")]
// pub struct AdminUserEditTemplate<'a> {
//     pub user_id: Option<&'a str>,

//     pub username: Option<&'a str>,
//     pub username_input_error: FieldError<'a>,

//     pub email: Option<&'a str>,
//     pub email_input_error: FieldError<'a>,

//     pub active: Option<bool>,

//     pub user_role_id: usize,

//     pub success_message: Option<&'a str>,
//     pub submit_url: &'a str,
//     pub user_roles: Vec<(i32, String)>,
// }

// impl<'a> AdminUserEditTemplate<'a> {
//     pub fn new_render_error(
//         user_roles: Vec<(i32, String)>,
//         user_id: Option<&'a str>,
//         username: Option<&'a str>,
//         username_input_error: FieldError<'a>,
//         email: Option<&'a str>,
//         email_input_error: FieldError<'a>,
//         active: Option<bool>,
//         user_role_id: Option<usize>,
//         submit_url: &'a str,
//     ) -> Result<String> {
//         Self {
//             user_roles,
//             user_id,
//             username,
//             username_input_error,
//             email,
//             email_input_error,
//             active,
//             user_role_id,
//             submit_url,
//             success_message: None,
//         }
//         .render()
//         .map_err(|_| Error::TemplateRenderingFailure)
//     }

//     pub fn new_render_existing(
//         user_roles: Vec<(i32, String)>,
//         user_id: &'a String,
//         username: Option<&'a str>,
//         email: &'a str,
//         active: bool,
//         user_role_id: usize,
//         submit_url: &'a str,
//         success_message: Option<&'a str>,
//     ) -> Result<String> {
//         Self {
//             user_roles,
//             user_id: Some(user_id.as_str()),
//             username,
//             username_input_error: None,
//             email: Some(email),
//             email_input_error: None,
//             active: Some(active),
//             user_role_id: Some(&user_role_id),
//             submit_url,
//             success_message,
//         }
//         .render()
//         .map_err(|_| Error::TemplateRenderingFailure)
//     }

//     pub fn new_render_blank(submit_url: &'a str, user_roles: Vec<(i32, String)>) -> Result<String> {
//         Self {
//             user_roles,
//             user_id: None,
//             username: None,
//             username_input_error: None,
//             email: None,
//             email_input_error: None,
//             active: None,
//             user_role_id: None,
//             submit_url,
//             success_message: None,
//         }
//         .render()
//         .map_err(|_| Error::TemplateRenderingFailure)
//     }
// }
