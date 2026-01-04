// lib.rs: Core library for the Meal Manager application

pub mod app;
pub mod dining_analysis;
pub mod db;
pub mod ui;

pub use app::{AppEvent, Action, AppState, EmployeeData, Page, Report};
pub use xilem::Xilem;
