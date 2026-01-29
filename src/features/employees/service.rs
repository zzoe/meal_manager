use super::actions::EmployeeAction;
use super::model::Employee;
use super::storage::EmployeeStorage;
use makepad_widgets::Cx;

pub fn load_config() {
    let employees = EmployeeStorage::load_employees();
    Cx::post_action(EmployeeAction::Loaded(employees));
}

pub fn save_config(employees: Vec<Employee>) {
    let _ = EmployeeStorage::save_employees(employees);
    Cx::post_action(EmployeeAction::Saved);
}

pub fn add_employee_config(emp: Employee) {
    let _ = EmployeeStorage::add_employee(emp);
    Cx::post_action(EmployeeAction::Saved);
}

pub fn update_employee_config(old_name: String, emp: Employee) {
    let _ = EmployeeStorage::update_employee(&old_name, emp);
    Cx::post_action(EmployeeAction::Saved);
}

pub fn delete_employee_config(name: String) {
    let _ = EmployeeStorage::delete_employee(&name);
    Cx::post_action(EmployeeAction::Saved);
}
