pub mod app_shell;
pub mod sidebar;

use makepad_widgets::Cx;

pub fn register_live_design(cx: &mut Cx) {
    sidebar::live_design(cx);
    app_shell::live_design(cx);
}
