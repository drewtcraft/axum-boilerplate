#[derive(Clone)]
pub struct ContextUserData {
    pub user_id: i64,
    pub email: String,
}

impl ContextUserData {
    pub fn new(user_id: i64, email: String) -> Self {
        Self { user_id, email }
    }
}

#[derive(Clone)]
pub struct Context {
    pub user_data: Option<ContextUserData>,
    pub is_htmx: Option<bool>,
}

impl Context {
    pub fn new(user_data: Option<ContextUserData>, is_htmx: Option<bool>) -> Self {
        Self { user_data, is_htmx }
    }
}
