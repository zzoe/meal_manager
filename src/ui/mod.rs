pub mod app;
pub mod pages;
pub mod theme;
pub mod layout;
pub mod components;
pub mod handlers;

use makepad_widgets::Cx;

pub fn register_live_design(cx: &mut Cx) {
    theme::register_live_design(cx);
    components::live_design(cx);
    layout::register_live_design(cx);
    pages::register_live_design(cx);
}