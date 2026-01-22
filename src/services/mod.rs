pub mod backend_result;
pub mod config_service;
pub mod meal_analysis;

pub use backend_result::BackendResult;
pub use config_service::{load_config, save_config};
pub use meal_analysis::analyze_meal;
