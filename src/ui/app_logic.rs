use crate::app::state::{AppState, Page};
use xilem::core::Edit;
use xilem::{Color, WidgetView};
use xilem::view::{button, flex_col, flex_row, label, sized_box, text_button, text_input};
use xilem::masonry::properties::types::Length;
use xilem::style::Style as _;
use xilem::view::FlexExt;

/// Main app logic that determines which page to render
pub fn app_logic(state: &mut AppState) -> impl WidgetView<Edit<AppState>> {
    let report = &state.current_report;
    let is_dining = state.current_page == Page::DiningStatistics;

    flex_col((
        // Status bar
        flex_col((
            flex_row((
                label(state.status_msg.clone())
                    .text_size(14.0)
                    .color(Color::from_rgb8(100, 100, 100)),
            )),
        ))
        .padding(5.0),
        // Content area
        if is_dining {
            // Dining Statistics
            flex_row((
                flex_col((
                    label("今日接龙数据:").text_size(16.0),
                    text_input(
                        state.input_text.clone(),
                        |state: &mut AppState, new_value| {
                            state.input_text = new_value;
                        },
                    )
                    .placeholder("输入接龙数据..."),
                    button(
                        label("立即分析"),
                        |state: &mut AppState| {
                            state
                                .tx_action
                                .send(crate::app::state::Action::Calculate(
                                    state.input_text.clone(),
                                ))
                                .unwrap();
                        },
                    ),
                ))
                .padding(10.0)
                .background_color(Color::from_rgb8(200, 230, 201)),
                // Report area
                flex_col((
                    label("分析报告").text_size(20.0),
                    card(
                        "🍱 中餐",
                        Color::from_rgb8(255, 247, 237),
                        format!("共 {} 份", report.lunch_total),
                        &report.lunch_details,
                    ),
                    card(
                        "🍲 晚餐",
                        Color::from_rgb8(239, 246, 255),
                        format!("共 {} 份", report.dinner_total),
                        &report.dinner_details,
                    ),
                    card(
                        "❌ 未报 / 异常",
                        Color::from_rgb8(254, 242, 242),
                        if !report.unknown.is_empty() {
                            format!("未知昵称: {}", report.unknown)
                        } else {
                            "".into()
                        },
                        &report.missing,
                    ),
                ))
                .gap(Length::px(10.0))
                .padding(15.0),
            ))
            .gap(Length::px(10.0))
            .padding(10.0)
        } else {
            // Settings
            sized_box(
                flex_col((
                    label("设置").text_size(24.0),
                    flex_col((
                        label("员工列表").text_size(18.0),
                        flex_col((
                            flex_row((
                                label("姓名:").text_size(14.0),
                                text_input(
                                    state.edit_name.clone(),
                                    |state: &mut AppState, new_value| {
                                        state.edit_name = new_value;
                                    },
                                )
                                .placeholder("输入姓名")
                                .flex(1.0),
                            ))
                            .gap(Length::px(5.0)),
                            flex_row((
                                label("昵称:").text_size(14.0),
                                text_input(
                                    state.edit_nicks.clone(),
                                    |state: &mut AppState, new_value| {
                                        state.edit_nicks = new_value;
                                    },
                                )
                                .placeholder("输入昵称,用逗号分隔")
                                .flex(1.0),
                            ))
                            .gap(Length::px(5.0)),
                            button(
                                label("添加员工"),
                                |state: &mut AppState| {
                                    if !state.edit_name.is_empty() {
                                        state
                                            .tx_action
                                            .send(crate::app::state::Action::SaveEmployee(
                                                crate::app::state::EmployeeData {
                                                    name: state.edit_name.clone(),
                                                    nicknames: state.edit_nicks.clone(),
                                                },
                                            ))
                                            .unwrap();
                                        state.edit_name = String::new();
                                        state.edit_nicks = String::new();
                                    }
                                },
                            ),
                        ))
                        .padding(15.0)
                        .background_color(Color::from_rgb8(245, 245, 245)),
                        flex_col(
                            state
                                .employees
                                .iter()
                                .map(|emp| {
                                    flex_col((
                                        flex_row((
                                            label(emp.name.clone()).text_size(16.0),
                                            label(emp.nicknames.clone())
                                                .text_size(14.0)
                                                .color(Color::from_rgb8(128, 128, 128)),
                                            text_button(
                                                "删除",
                                                move |state: &mut AppState| {
                                                    state
                                                        .tx_action
                                                        .send(
                                                            crate::app::state::Action::DeleteEmployee(
                                                                emp.name.clone(),
                                                            ),
                                                        )
                                                        .unwrap();
                                                },
                                            ),
                                        ))
                                        .main_axis_alignment(
                                            xilem::masonry::properties::types::MainAxisAlignment::SpaceBetween,
                                        ),
                                    ))
                                    .padding(10.0)
                                    .background_color(Color::from_rgb8(255, 255, 255))
                                })
                                .collect::<Vec<_>>(),
                        )
                        .gap(Length::px(5.0)),
                    ))
                    .gap(Length::px(10.0))
                    .padding(15.0),
                ))
                .gap(Length::px(15.0))
                .padding(20.0),
            )
        },
        // Navigation bar
        flex_col((
            text_button(
                "就餐统计",
                |state: &mut AppState| {
                    state.current_page = Page::DiningStatistics;
                },
            ),
            text_button(
                "设置",
                |state: &mut AppState| {
                    state.current_page = Page::Settings;
                },
            ),
        ))
        .gap(Length::px(5.0))
        .padding(10.0)
        .background_color(Color::from_rgb8(240, 240, 240)),
    ))
    .gap(Length::px(10.0))
}

/// A helper function to create a card component for the report
fn card(
    title: &str,
    bg: Color,
    subtitle: String,
    content: &str,
) -> impl WidgetView<Edit<AppState>> {
    flex_col((
        flex_row((
            label(title),
            label(subtitle).color(Color::from_rgb8(128, 128, 128)),
        ))
        .main_axis_alignment(
            xilem::masonry::properties::types::MainAxisAlignment::SpaceBetween,
        )
        .padding(10.0),
        label(content).text_size(13.0),
    ))
    .padding(10.0)
    .gap(Length::px(5.0))
    .background_color(bg)
}
