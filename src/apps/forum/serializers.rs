use serde::Deserialize;

use crate::traits::{ParamValidationError, ParamValidator};

#[derive(Clone, Debug, Deserialize, Default)]
pub struct NewThread {
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct NewThreadErrors {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl ParamValidationError for NewThreadErrors {
    fn new_empty() -> Self {
        Self {
            title: None,
            content: None,
        }
    }
}

impl ParamValidator<NewThreadErrors> for NewThread {
    fn validate(&self) -> (bool, NewThreadErrors) {
        let mut valid = true;
        let mut errors = NewThreadErrors::new_empty();
        if self.title.len() > 140 {
            valid = false;
            errors.title = Some("Title cannot be longer than 140 characters.".to_string());
        } else if self.title.trim().is_empty() {
            valid = false;
            errors.title = Some("Title cannot be blank".to_string());
        }

        if self.content.trim().is_empty() {
            valid = false;
            errors.content = Some("content cannot be blank".to_string())
        }

        (valid, errors)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct PathId {
    pub id: i64,
}
