//! 平台特定功能模块
//!
//! 此模块包含各平台特定的实现，通过条件编译只在对应平台生效

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;
