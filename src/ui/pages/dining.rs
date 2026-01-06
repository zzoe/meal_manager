use crate::app::AppState;
use crate::ui::CJK_FONT_STACK;
use xilem::masonry::properties::types::{Length, MainAxisAlignment};
use xilem::style::Style;
use xilem::view::{FlexExt, button, flex_col, flex_row, label, text_input};
use xilem::{Color, WidgetView};
use xilem_core::Edit;

pub fn dining_page(state: &mut AppState) -> impl WidgetView<Edit<AppState>> + use<> {
    let report = &state.current_report;
    flex_row((
        // 左侧输入区
        flex_col((
            label("今日接龙数据:")
                .font(CJK_FONT_STACK)
                .color(Color::BLACK),
            text_input(
                state.input_text.clone(),
                |state: &mut AppState, new_value| {
                    state.input_text = new_value;
                },
            )
            .font(CJK_FONT_STACK)
            .flex(1.0),
            button(
                label("立即分析").font(CJK_FONT_STACK),
                |state: &mut AppState| {
                    state.status.message = "计算中...".into();
                    if let Err(e) = state
                        .tx_action
                        .send(crate::app::Action::Calculate(state.input_text.clone()))
                    {
                        state.status.message = format!("发送计算请求失败: {}", e);
                    }
                },
            ),
        ))
        .gap(Length::px(10.0))
        .padding(10.0)
        .background_color(Color::from_rgb8(200, 230, 201))
        .flex(1.0),
        // 右侧报表区 - 占满剩余空间，每个卡片最大宽度
        flex_col((
            label("分析报告").font(CJK_FONT_STACK),
            card(
                "🍱 中餐",
                Color::from_rgb8(255, 247, 237),
                format!("共 {} 份", report.lunch_total),
                &report.lunch_details,
            )
            .flex(1.0),
            card(
                "🥗 晚餐",
                Color::from_rgb8(237, 247, 255),
                format!("共 {} 份", report.dinner_total),
                &report.dinner_details,
            )
            .flex(1.0),
            card(
                "❓ 失踪",
                Color::from_rgb8(255, 237, 237),
                "未报餐".to_string(),
                &report.missing,
            )
            .flex(1.0),
            card(
                "❔ 未知",
                Color::from_rgb8(245, 245, 245),
                "格式错误".to_string(),
                &report.unknown,
            )
            .flex(1.0),
        ))
        .gap(Length::px(10.0))
        .padding(15.0)
        .flex(2.0),
    ))
    .gap(Length::px(10.0))
    .padding(10.0)
}

fn card(
    title: &str,
    bg: Color,
    subtitle: String,
    content: &str,
) -> impl WidgetView<Edit<AppState>> + use<> {
    flex_col((
        flex_row((
            label(title).font(CJK_FONT_STACK).color(Color::BLACK),
            label(subtitle).font(CJK_FONT_STACK).color(Color::BLACK),
        ))
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(10.0),
        xilem::view::portal(cjk_prose(content, 13.0)),
    ))
    .gap(Length::px(5.0))
    .padding(10.0)
    .background_color(bg)
}

fn cjk_prose(content: &str, size: f32) -> impl WidgetView<Edit<AppState>> + use<> {
    flex_col(
        content
            .lines()
            .map(|line| {
                label(line)
                    .font(CJK_FONT_STACK)
                    .text_size(size)
                    .color(Color::BLACK)
                    .line_break_mode(xilem::masonry::properties::LineBreaking::WordWrap)
            })
            .collect::<Vec<_>>(),
    )
}
