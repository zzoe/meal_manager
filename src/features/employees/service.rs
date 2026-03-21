use super::actions::{EmployeeAction, EmployeeOp};
use super::model::Employee;
use super::storage::EmployeeStorage;
use makepad_widgets::Cx;

pub fn load_config() {
    let employees = EmployeeStorage::load_employees();
    Cx::post_action(EmployeeAction::Loaded(employees));
}

pub fn add_employee_config(emp: Employee) {
    if let Err(err) = EmployeeStorage::add_employee(emp.clone()) {
        Cx::post_action(EmployeeAction::SaveFailed {
            op: EmployeeOp::Add,
            name: emp.name,
            error: err.to_string(),
        });
    }
}

pub fn update_employee_config(old_name: String, emp: Employee) {
    if let Err(err) = EmployeeStorage::update_employee(&old_name, emp.clone()) {
        Cx::post_action(EmployeeAction::SaveFailed {
            op: EmployeeOp::Update,
            name: format!("{old_name} -> {}", emp.name),
            error: err.to_string(),
        });
    }
}

pub fn delete_employee_config(name: String) {
    if let Err(err) = EmployeeStorage::delete_employee(&name) {
        Cx::post_action(EmployeeAction::SaveFailed {
            op: EmployeeOp::Delete,
            name,
            error: err.to_string(),
        });
    }
}
