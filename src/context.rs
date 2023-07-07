#[derive(Clone, Debug)]
pub struct ContextUserData {
    pub user_id: i64,
    pub email: String,
    pub session_uid: String,
}

impl ContextUserData {
    pub fn new(user_id: i64, email: String, session_uid: String) -> Self {
        Self {
            user_id,
            email,
            session_uid,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub user_data: Option<ContextUserData>,
    pub is_htmx: Option<bool>,
    pub page_title: Option<String>,
}

impl Context {
    pub fn new(user_data: Option<ContextUserData>, is_htmx: Option<bool>) -> Self {
        Self { user_data, is_htmx, page_title: None }
    }

    pub fn set_is_htmx(&mut self, is_htmx: bool) {
        self.is_htmx = Some(is_htmx);
    }

    pub fn set_page_title(&mut self, page_title: &str) {
        self.page_title = Some(page_title.to_string());
    }
}
