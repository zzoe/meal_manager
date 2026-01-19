pub mod config_page;
pub mod stats_page;

use makepad_widgets::Cx;

pub fn register_live_design(cx: &mut Cx) {
    stats_page::live_design(cx);
    config_page::live_design(cx);
}
