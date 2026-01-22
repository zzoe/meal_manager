pub mod config_parser;
pub mod database;

pub use config_parser::{format_employee_config, parse_employee_config};
pub use database::{DatabaseConnection, get_database};
