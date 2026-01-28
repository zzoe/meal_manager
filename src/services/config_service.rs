use crate::services::backend_result::BackendResult;
use crate::storage::{DatabaseConnection, format_employee_config, parse_employee_config};
use makepad_widgets::Cx;

pub fn load_config() {
    let list = DatabaseConnection::load_employees();
    let text = format_employee_config(&list);
    Cx::post_action(BackendResult::ConfigLoaded(text));
}

pub fn save_config(text: String) {
    let list = parse_employee_config(&text);
    let _ = DatabaseConnection::save_employees(list);
    Cx::post_action(BackendResult::ConfigSaved);
}
