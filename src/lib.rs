pub mod features;
#[cfg(not(target_arch = "wasm32"))]
pub mod infra;
pub mod ui;

pub use features::employees;
pub use features::meal_stats;
pub use ui::app;
