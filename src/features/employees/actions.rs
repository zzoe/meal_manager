use crate::employees::Employee;
use makepad_widgets::{ActionDefaultRef, DefaultNone};

#[derive(Clone, Debug, DefaultNone)]
pub enum EmployeeAction {
    Loaded(Vec<Employee>),
    Saved,
    None,
}
