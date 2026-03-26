pub mod employees;
pub mod meal_stats;

// 页面模块导出
pub use employees::{EmployeePage, EmployeePageAction, EmployeePageRef};
pub use meal_stats::{StatsPage, StatsPageRef};

use makepad_widgets::Cx;

pub fn register_live_design(cx: &mut Cx) {
    meal_stats::live_design(cx);
    employees::live_design(cx);
}
