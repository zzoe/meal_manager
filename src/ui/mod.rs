pub mod app;
pub mod components;
pub mod layout;
pub mod pages;

use makepad_widgets::Cx;

pub fn register_live_design(cx: &mut Cx) {
    println!("UI: register_live_design start");
    components::live_design(cx);
    pages::register_live_design(cx);
    layout::register_live_design(cx);
    println!("UI: register_live_design end");
}
