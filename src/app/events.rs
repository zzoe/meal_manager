use crate::app::state::{EmployeeData, Report};

/// 后台发给 UI 的事件
#[derive(Clone, Debug)]
pub enum AppEvent {
    EmployeesLoaded(Vec<EmployeeData>),
    ReportReady(Report),
    StatusMessage(String),
}
