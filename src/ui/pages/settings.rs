use crate::app::{Action, AppState, EmployeeData};
use crate::ui::CJK_FONT_STACK;
use xilem::masonry::properties::types::{Length, MainAxisAlignment};
use xilem::style::Style;
use xilem::view::{FlexExt, button, flex_col, flex_row, label, sized_box, text_input};
use xilem::{Color, WidgetView};
use xilem_core::Edit;

pub fn settings_page(state: &mut AppState) -> impl WidgetView<Edit<AppState>> + use<> {
    sized_box(
        flex_col((
            label("设置").font(CJK_FONT_STACK).text_size(24.0),
            // 员工管理表格
            flex_col((
                label("员工列表").font(CJK_FONT_STACK).text_size(18.0),
                // 表头
                employee_table_header(),
                //现有员工行
                flex_col(
                    state
                        .employees
                        .iter()
                        .map(|emp| {
                            employee_row_editable(
                                emp.clone(),
                                state.editing.original_name.clone(),
                                state.editing.name.clone(),
                                state.editing.nicknames.clone(),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
                .gap(Length::px(5.0)),
                // 新增行（空输入框 + 添加按钮）
                employee_edit_row_with_add(state.editing.name.clone(), state.editing.nicknames.clone()),
            ))
            .gap(Length::px(10.0))
            .padding(15.0)
            .background_color(Color::from_rgb8(245, 245, 245)),
            // 退出按钮
            button(
                label("退出程序").font(CJK_FONT_STACK),
                |state: &mut AppState| {
                    state.status.is_running = false;
                },
            )
            .background_color(Color::from_rgb8(255, 200, 200)),
        ))
        .gap(Length::px(15.0))
        .padding(20.0),
    )
}

fn employee_table_header() -> impl WidgetView<Edit<AppState>> + use<> {
    flex_row((
        label("姓名")
            .font(CJK_FONT_STACK)
            .text_size(14.0)
            .color(Color::from_rgb8(80, 80, 80)),
        label("昵称")
            .font(CJK_FONT_STACK)
            .text_size(14.0)
            .color(Color::from_rgb8(80, 80, 80)),
        label("操作")
            .font(CJK_FONT_STACK)
            .text_size(14.0)
            .color(Color::from_rgb8(80, 80, 80)),
    ))
    .main_axis_alignment(MainAxisAlignment::SpaceBetween)
    .padding(8.0)
    .background_color(Color::from_rgb8(230, 230, 230))
}

fn employee_row_editable(
    emp: EmployeeData,
    state_editing: Option<String>,
    state_edit_name: String,
    state_edit_nicks: String,
) -> impl WidgetView<Edit<AppState>> + use<> {
    let name = emp.name.clone();
    let nicks = emp.nicknames.clone();
    let is_editing = state_editing.as_ref() == Some(&name);

    let name_for_input1 = name.clone();
    let name_for_input2 = name.clone();
    let name_for_edit = name.clone();
    let nicks_for_edit = nicks.clone();
    let name_for_delete = name.clone();

    flex_row((
        text_input(
            if is_editing {
                state_edit_name.clone()
            } else {
                name.clone()
            },
            move |state: &mut AppState, val| {
                if state.editing.original_name.as_ref() == Some(&name_for_input1) {
                    state.editing.name = val;
                }
            },
        )
        .font(CJK_FONT_STACK)
        .text_size(14.0)
        .flex(1.0),
        text_input(
            if is_editing {
                state_edit_nicks.clone()
            } else {
                nicks.clone()
            },
            move |state: &mut AppState, val| {
                if state.editing.original_name.as_ref() == Some(&name_for_input2) {
                    state.editing.nicknames = val;
                }
            },
        )
        .font(CJK_FONT_STACK)
        .text_size(14.0)
        .flex(1.0),
        if is_editing {
            // 保存按钮
            button(
                label("保存").font(CJK_FONT_STACK),
                move |state: &mut AppState| {
                    if let Some(old_name) = state.editing.original_name.take() {
                        if let Err(e) = state
                            .tx_action
                            .send(Action::UpdateEmployee {
                                old_name,
                                new_data: EmployeeData {
                                    name: state.editing.name.clone(),
                                    nicknames: state.editing.nicknames.clone(),
                                },
                            })
                        {
                            state.status.message = format!("发送更新请求失败: {}", e);
                        }
                        state.editing.name.clear();
                        state.editing.nicknames.clear();
                        state.editing.is_adding_new = true;
                    }
                },
            )
            .background_color(Color::from_rgb8(200, 255, 200))
            .boxed()
        } else {
            // 编辑按钮
            button(
                label("编辑").font(CJK_FONT_STACK),
                move |state: &mut AppState| {
                    state.editing.original_name = Some(name_for_edit.clone());
                    state.editing.name = name_for_edit.clone();
                    state.editing.nicknames = nicks_for_edit.clone();
                    state.editing.is_adding_new = false;
                },
            )
            .background_color(Color::from_rgb8(200, 220, 255))
            .boxed()
        },
        button(
            label("删除").font(CJK_FONT_STACK),
            move |state: &mut AppState| {
                if let Err(e) = state
                    .tx_action
                    .send(Action::DeleteEmployee(name_for_delete.clone()))
                {
                    state.status.message = format!("发送删除请求失败: {}", e);
                }
            },
        )
        .background_color(Color::from_rgb8(255, 200, 200)),
    ))
    .gap(Length::px(8.0))
    .padding(8.0)
    .background_color(Color::from_rgb8(255, 255, 255))
}

fn employee_edit_row_with_add(
    name: String,
    nicks: String,
) -> impl WidgetView<Edit<AppState>> + use<> {
    flex_row((
        text_input(name, |state: &mut AppState, val| {
            state.editing.name = val;
        })
        .font(CJK_FONT_STACK)
        .text_size(14.0)
        .flex(1.0),
        text_input(nicks, |state: &mut AppState, val| {
            state.editing.nicknames = val;
        })
        .font(CJK_FONT_STACK)
        .text_size(14.0)
        .flex(1.0),
        button(
            label("添加").font(CJK_FONT_STACK),
            |state: &mut AppState| {
                if !state.editing.name.is_empty() {
                    if let Err(e) = state
                        .tx_action
                        .send(Action::SaveEmployee(EmployeeData {
                            name: state.editing.name.clone(),
                            nicknames: state.editing.nicknames.clone(),
                        }))
                    {
                        state.status.message = format!("发送添加请求失败: {}", e);
                    }
                    state.editing.reset();
                }
            },
        )
        .background_color(Color::from_rgb8(200, 255, 200)),
    ))
    .gap(Length::px(8.0))
    .padding(8.0)
    .background_color(Color::from_rgb8(255, 255, 255))
}