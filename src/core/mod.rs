//! 业务逻辑主模块 - 包含所有与报餐管理相关的业务逻辑

pub mod db;
pub mod employee;
pub mod dining_analysis;

// 重新导出常用类型
pub use db::DbManager;
pub use employee::{EmployeeData, EmployeeManager};
pub use dining_analysis::{Report, DiningAnalyzer};