use makepad_widgets::Cx;

pub mod employee_item;
pub mod employee_page;

// 向后兼容的别名
pub use employee_item::EmployeeItem;
pub use employee_item::EmployeeItemAction;
pub use employee_item::EmployeeItemRef;
pub use employee_page::EmployeePage;
pub use employee_page::EmployeePageAction;
pub use employee_page::EmployeePageRef;

pub fn live_design(cx: &mut Cx) {
    employee_item::live_design(cx);
    employee_page::live_design(cx);
}
