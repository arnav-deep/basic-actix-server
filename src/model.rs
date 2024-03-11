use std::sync::Mutex;

pub struct AppState {
    pub app_name: String,
    pub counter: Mutex<i32>,
}
