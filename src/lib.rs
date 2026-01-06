//! Meal Manager 报餐助手 Pro
//! 
//! 一个用于分析和管理团队报餐数据的桌面应用程序
//! 
//! ## 架构概述
//! - `app/`: 应用状态和事件管理
//! - `db/`: 数据库操作模块
//! - `dining_analysis/`: 报餐数据分析逻辑
//! - `ui/`: 用户界面组件
//! - `error/`: 统一错误处理

pub mod app;
pub mod db;
pub mod dining_analysis;
pub mod error;
pub mod ui;

pub use app::{Action, AppEvent, AppState, AppStatus, EditingState, EmployeeData, Page, Report};
pub use error::{AppError, DatabaseError, AnalysisError, Result};
pub use xilem::Xilem;

/// 常用类型的预导入模块
pub mod prelude {
    pub use crate::app::{Action, AppEvent, AppState, AppStatus, EditingState, EmployeeData, Page, Report};
    pub use crate::error::{AppError, DatabaseError, AnalysisError, Result};
}
