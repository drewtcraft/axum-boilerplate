use super::serializers::{UserListParams, UserListParamsErrors, UserEditParams, UserEditParamsErrors};
use crate::apps::user::models::User;
use crate::error::{Error, Result};
use crate::traits::{ParamValidationError, ToPlainText};
use sailfish::TemplateOnce;
use crate::templates::BaseTemplate;

// --------------
// USER TEMPLATES
// --------------
#[derive(TemplateOnce)]
#[template(path = "log-in.stpl")]
pub struct LogInTemplate {
    pub input_error: Option<String>,
}

impl LogInTemplate {
    pub fn new_render(is_htmx: bool) -> Result<String> {
        let rendered = Self { input_error: None }
            .render_once()
            .map_err(|_| Error::TemplateRenderingFailure)?;

        if is_htmx {
            Ok(rendered)
        } else {
            BaseTemplate::new_render(rendered)
        }
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

#[derive(Clone)]
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
    pub users_list: Vec<UserListUser>,
    pub user_roles: Vec<(i32, String)>,
    pub query_params: UserListParams,
    pub query_params_errors: UserListParamsErrors,
}

impl AdminUserListTemplate {
    pub fn new_render(
        users_list: Vec<User::User>,
        user_roles: Vec<(i32, String)>,
        query_params: UserListParams,
    ) -> Result<String> {
        let formatted_users: Vec<UserListUser> = users_list
            .iter()
            .map(|user| UserListUser {
                id: user.id,
                username: user.username.clone(),
                email: user.email.clone(),
                active: user.active,
                user_role_id: user.user_role_id as usize,
                created_at: user.created_at.clone(),
                updated_at: user.updated_at.clone(),
            })
            .collect();

        Self {
            users_list: formatted_users,
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
            users_list: vec![],
            user_roles,
            query_params,
            query_params_errors,
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)
    }
}

#[derive(TemplateOnce)]
#[template(path = "admin_user-edit.stpl")]
pub struct AdminUserEditTemplate {
    pub user_id: Option<String>,
    pub user_roles: Vec<(i32, String)>,
    pub query_params: UserEditParams,
    pub query_params_errors: UserEditParamsErrors,
    pub success_message: Option<String>,
    pub submit_url: String,
}

impl AdminUserEditTemplate {
    pub fn new_render_error(
        user_id: Option<String>,
        user_roles: Vec<(i32, String)>,
        query_params: UserEditParams,
        query_params_errors: UserEditParamsErrors,
        submit_url: String,
    ) -> Result<String> {
        Self {
            user_id,
            user_roles,
            query_params,
            query_params_errors,
            submit_url,
            success_message: None,
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)
    }

    pub fn new_render_existing(
        user_id: String,
        user_roles: Vec<(i32, String)>,
        query_params: UserEditParams,
        submit_url: String,
        success_message: Option<String>,
    ) -> Result<String> {
        Self {
            user_id: Some(user_id),
            user_roles,
            query_params,
            query_params_errors: UserEditParamsErrors::new_empty(),
            submit_url,
            success_message,
        }
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)
    }
}
