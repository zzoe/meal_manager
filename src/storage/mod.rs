pub mod database;
pub mod config_parser;

pub use database::{DatabaseConnection, get_database};
pub use config_parser::{parse_employee_config, format_employee_config};
