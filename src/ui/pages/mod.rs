pub mod employees;
pub mod meal_stats;

// Re-export for backward compatibility (optional, but good for now)
pub use employees::page as config_page;
pub use meal_stats as stats_page;

use makepad_widgets::Cx;

pub fn register_live_design(cx: &mut Cx) {
    meal_stats::live_design(cx);
    employees::live_design(cx);
}
