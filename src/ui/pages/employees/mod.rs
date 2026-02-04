use makepad_widgets::Cx;

pub mod config_item;
pub mod page;

pub fn live_design(cx: &mut Cx) {
    config_item::live_design(cx);
    page::live_design(cx);
}
