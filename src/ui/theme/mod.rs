use makepad_widgets::*;

pub mod styles;

pub fn register_live_design(cx: &mut Cx) {
    styles::live_design(cx);
}
