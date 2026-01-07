//! Meal Manager 报餐助手 Pro
//! 
//! 一个用于分析和管理团队报餐数据的桌面应用程序

pub use makepad_widgets;

// 业务逻辑模块（非UI相关）
pub mod core;

// UI相关模块
pub mod ui;

// 重新导出常用类型
pub use core::{EmployeeData, EmployeeManager, Report, DiningAnalyzer};

/// 常用类型的预导入模块
pub mod prelude {
    pub use crate::core::{EmployeeData, EmployeeManager, Report, DiningAnalyzer};
}