use crate::employees::Employee;
use makepad_widgets::{ActionDefaultRef, DefaultNone};

#[derive(Clone, Debug)]
pub enum EmployeeOp {
    Add,
    Update,
    Delete,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum EmployeeAction {
    Loaded(Vec<Employee>),
    SaveFailed {
        op: EmployeeOp,
        name: String,
        error: String,
    },
    None,
}
