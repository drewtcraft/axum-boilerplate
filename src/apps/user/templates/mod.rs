use crate::error::{Error, Result};
use crate::traits::ToPlainText;
use askama::Template;
use super::serializers::{UserListParams, UserListParamsErrors};
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

// pub type FieldError<'a> = Option<&'a str>;

// pub struct UserListUser<'a> {
//     pub id: i64,
//     pub username: Option<&'a str>,
//     pub email: &'a str,
//     pub active: bool,
//     pub user_role_id: usize,
//     pub created_at: &'a str,
//     pub updated_at: &'a str,
// }

// #[derive(Template)]
// #[template(path = "admin_user-list.html")]
// pub struct AdminUserListTemplate<'a> {
//     pub users_list: Option<Vec<UserListUser<'a>>>,

//     pub user_roles: Vec<(i32, String)>,

//     pub user_id_input: Option<String>,
//     pub user_id_input_error: FieldError<'a>,

//     pub username_input: Option<String>,
//     pub username_input_error: FieldError<'a>,

//     pub active_input: Option<String>,

//     pub email_input: Option<String>,
//     pub email_input_error: FieldError<'a>,

//     pub user_role_id_input: usize,

//     pub sort_by: Option<String>,  // username, email
//     pub sort_dir: Option<String>, // ASC, DESC
// }

// impl AdminUserListTemplate<'a> {
//     pub fn new_render(
//         users_list: Option<Vec<UserListUser<'a>>>,
//         user_roles: Vec<(i32, String)>,
//         query_params: UserListParams,
//     ) -> Result<String> {
//         Self {
//             users_list,
//             user_roles,
//             user_id_input: query_params.user_id.as_ref().map(|s| s.as_str()),
//             user_id_input_error: None,
//             username_input: query_params.username.as_ref().map(|s| s.as_str()),
//             username_input_error: None,
//             active_input: query_params.active.as_ref().map(|s| s.as_str()),
//             email_input: query_params.email.as_ref().map(|s| s.as_str()),
//             email_input_error: None,
//             user_role_id_input: query_params.user_role_id.unwrap_or(0) as usize,
//             sort_by: query_params.sort_by.as_ref().map(|s| s.as_str()),
//             sort_dir: query_params.sort_dir.as_ref().map(|s| s.as_str()),
//         }
//         .render()
//         .map_err(|_| Error::TemplateRenderingFailure)
//     }

//     pub fn new_render_error(
//         user_roles: Vec<(i32, String)>,
//         query_params: UserListParams,
//         query_params_errors: UserListParamsErrors,
//     ) -> Result<String> {
//         Self {
//             users_list: None,
//             user_roles,
//             user_id_input: query_params.user_id.as_ref().map(|s| s.as_str()),
//             user_id_input_error: query_params_errors.user_id.as_ref().map(|s| s.as_str()),
//             username_input: query_params.username.as_ref().map(|s| s.as_str()),
//             username_input_error: query_params_errors.username.as_ref().map(|s| s.as_str()),
//             active_input: query_params.active.as_ref().map(|s| s.as_str()),
//             email_input: query_params.email.as_ref().map(|s| s.as_str()),
//             email_input_error: query_params_errors.email.as_ref().map(|s| s.as_str()),
//             user_role_id_input: query_params.user_role_id.unwrap_or(0) as usize,
//             sort_by: query_params.sort_by.as_ref().map(|s| s.as_str()),
//             sort_dir: query_params.sort_dir.as_ref().map(|s| s.as_str()),
//         }
//         .render()
//         .map_err(|_| Error::TemplateRenderingFailure)
//     }
// }

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
