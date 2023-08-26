use serde::Deserialize;

use crate::{
    traits::{ParamValidationError, ParamValidator},
    utils::{ALPHANUMERIC_UNDERSCORE_RX, SIMPLE_EMAIL_RX},
};

#[derive(Clone, Deserialize)]
pub struct UidParam {
    pub uid: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LogInBody {
    pub username_or_email: String,
}

#[derive(Clone, Deserialize)]
pub struct SignUpBody {
    pub email: String,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SendInviteBody {
    pub email: String,
}

#[derive(Clone, Deserialize)]
pub struct IdParam {
    pub id: i64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UserEditParams {
    pub username: Option<String>,
    pub email: String,
    pub active: bool,
    pub role: i32,
}

pub struct UserEditParamsErrors {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

impl ParamValidationError for UserEditParamsErrors {
    fn new_empty() -> Self {
        Self {
            username: None,
            email: None,
            role: None,
        }
    }
}

impl ParamValidator<UserEditParamsErrors> for UserEditParams {
    fn validate(&self) -> (bool, UserEditParamsErrors) {
        let mut valid = true;
        let mut errors = UserEditParamsErrors::new_empty();

        if let Some(username) = self.username.clone() {
            if !username.is_empty() {
                if !ALPHANUMERIC_UNDERSCORE_RX.is_match(&username.trim()) {
                    valid = false;
                    errors.username = Some("username is alphanumeric plus \"_\"".to_string());
                }
            }
        }

        if !&self.email.is_empty() {
            if !SIMPLE_EMAIL_RX.is_match(&self.email.trim()) {
                valid = false;
                errors.email = Some("email does not look like an email...".to_string());
            }
        }

        (valid, errors)
    }
}

#[derive(Clone, Deserialize)]
pub struct UserListParams {
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub active: Option<String>,
    pub user_role_id: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

pub struct UserListParamsErrors {
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}

impl ParamValidationError for UserListParamsErrors {
    fn new_empty() -> Self {
        Self {
            user_id: None,
            username: None,
            email: None,
        }
    }
}

impl ParamValidator<UserListParamsErrors> for UserListParams {
    fn validate(&self) -> (bool, UserListParamsErrors) {
        let mut valid = true;
        let mut errors = UserListParamsErrors::new_empty();

        if let Some(user_id) = self.user_id.clone() {
            if !user_id.is_empty() {
                match user_id.parse::<i64>() {
                    Ok(n) => {
                        if n <= 0 {
                            valid = false;
                            errors.user_id = Some("user id must be greater than 1".to_string());
                        }
                    }
                    Err(_) => {
                        valid = false;
                        errors.user_id = Some("user id must be an integer".to_string());
                    }
                }
            }
        }

        if let Some(username) = self.username.clone() {
            if !username.is_empty() {
                if !ALPHANUMERIC_UNDERSCORE_RX.is_match(&username.trim()) {
                    valid = false;
                    errors.username = Some("username is alphanumeric plus \"_\"".to_string());
                }
            }
        }

        if let Some(email) = self.email.clone() {
            if !email.is_empty() {
                if !SIMPLE_EMAIL_RX.is_match(&email.trim()) {
                    valid = false;
                    errors.email = Some("email does not look like an email...".to_string());
                }
            }
        }

        (valid, errors)
    }
}
