use crate::app::{AppState, Page};
use crate::ui::CJK_FONT_STACK;
use crate::ui::pages;
use xilem::masonry::properties::types::Length;
use xilem::style::Style;
use xilem::view::{FlexExt, button, flex_col, flex_row, label};
use xilem::{Color, WidgetView, window};

pub fn app_logic(state: &mut AppState) -> std::iter::Once<xilem::WindowView<AppState>> {
    state.tick();
    let content = match state.current_page {
        Page::DiningStatistics => pages::dining::dining_page(state).boxed(),
        Page::Settings => pages::settings::settings_page(state).boxed(),
    };

    // 图标导航
    let nav_icon = if state.current_page == Page::DiningStatistics {
        button(
            label("⚙️").font(CJK_FONT_STACK).text_size(24.0),
            |state: &mut AppState| {
                state.current_page = Page::Settings;
            },
        )
        .boxed()
    } else {
        button(
            label("🏠").font(CJK_FONT_STACK).text_size(24.0),
            |state: &mut AppState| {
                state.current_page = Page::DiningStatistics;
            },
        )
        .boxed()
    };

    let main_view = flex_col((
        // 顶部栏：图标 + 状态
        flex_row((
            nav_icon,
            label(state.status.message.clone())
                .font(CJK_FONT_STACK)
                .text_size(14.0)
                .color(Color::from_rgb8(100, 100, 100))
                .flex(1.0),
        ))
        .padding(5.0)
        .gap(Length::px(10.0)),
        // 可滚动内容区域
        xilem::view::portal(content),
    ))
    .gap(Length::px(10.0));

    std::iter::once(
        window(state.status.window_id, "报餐助手 Pro", main_view).with_options(|_state| {
            xilem::WindowOptions::new("报餐助手 Pro").on_close(|state: &mut AppState| {
                state.status.is_running = false;
            })
        }),
    )
}