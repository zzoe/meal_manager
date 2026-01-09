// pub mod meal_statistics;
pub mod meal_view;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    meal_view::live_design(cx);
}
