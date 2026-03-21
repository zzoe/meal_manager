use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    // 结果展示卡片
    pub ResultCard = <RoundedView> {
        width: Fill, height: Fill
        flow: Down, spacing: 0.0

        draw_bg: {
            color: (THEME_COLOR_WHITE)
            border_color: (THEME_COLOR_INSET_1)
            border_size: 1.0
            border_radius: 8.0
        }

        // 标题栏 (带背景色)
        header_slot = <RoundedYView> {
            width: Fill, height: 40.0
            padding: {top: 10.0, left: 15.0}
            draw_bg: {
                color: (THEME_COLOR_LIGHT)
                border_radius: vec2( 8.0, 1.0)
            }

            header_label = <Label> {
                text: "Title"
                draw_text: {
                    text_style: { font_size: 12.0 }
                }
            }
        }

        // 内容区 (白色背景 + 滚动)
        content_view = <ScrollYView> {
            width: Fill, height: Fill
            padding: 15.0

            content = <Label> {
                width: Fill, height: Fit
                text: "暂无数据"
                draw_text: {
                    color: (THEME_COLOR_TEXT)
                    wrap: Word
                    text_style: { font_size: 13.0 }
                }
            }
        }
    }

    // 侧边栏导航按钮
    pub NavButton = <Button> {
        width: Fill, height: 40.0
        draw_text: {
            color: (THEME_COLOR_TEXT)
            color_hover: (THEME_COLOR_TEXT)
            color_down: (THEME_COLOR_TEXT)
            color_focus: (THEME_COLOR_TEXT)
            color_disabled: (THEME_COLOR_TEXT_DISABLED)
            text_style: { font_size: 12.0 }
        }
        draw_bg: {
            color: (THEME_COLOR_U_HIDDEN)
            border_size: 0.0
            border_radius: 4.0
            color_hover: (THEME_COLOR_U_4)
            color_down: (THEME_COLOR_U_6)

            // 确保状态切换时颜色正确
            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
        }
    }

    // 侧边栏折叠按钮 (小正方形)
    pub ToggleButton = <Button> {
        width: Fill, height: 30.0
        draw_text: {
            color: (THEME_COLOR_TEXT)
            color_hover: (THEME_COLOR_TEXT)
            color_down: (THEME_COLOR_TEXT)
            color_focus: (THEME_COLOR_TEXT)
            color_disabled: (THEME_COLOR_TEXT_DISABLED)
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: (THEME_COLOR_U_HIDDEN)
            border_size: 0.0
            color_hover: (THEME_COLOR_U_4)
            color_down: (THEME_COLOR_U_6)
        }
    }

    // 蓝色按钮
    pub BlueButton = <Button> {
        width: Fill, height: 48.0
        draw_text: {
            color: (THEME_COLOR_WHITE)
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: (THEME_COLOR_VAL)
            color_hover: (THEME_COLOR_VAL_HOVER)
            color_down: (THEME_COLOR_VAL_FOCUS)
            color_focus: (THEME_COLOR_VAL)

            border_size: 0.0
            border_radius: 6.0

            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
        }
    }

    // 带样式的输入框 (用于表格或卡片)
    pub StyledTextInput = <TextInput> {
        width: Fill, height: Fit
        padding: {left: 10.0, right: 10.0, top: 4.0, bottom: 4.0}

        draw_bg: {
            color: (THEME_COLOR_LIGHT)
            border_size: 1.0
            border_color_1: (THEME_COLOR_INSET_1)
            border_color_1_hover: (THEME_COLOR_INSET_1)
            border_color_1_focus: (THEME_COLOR_INSET_1)
            border_color_1_down: (THEME_COLOR_INSET_1)
            border_color_1_empty: (THEME_COLOR_INSET_1)
            border_color_1_disabled: (THEME_COLOR_INSET_1)
            border_color_2: (THEME_COLOR_INSET_1)
            border_color_2_hover: (THEME_COLOR_INSET_1)
            border_color_2_focus: (THEME_COLOR_INSET_1)
            border_color_2_down: (THEME_COLOR_INSET_1)
            border_color_2_empty: (THEME_COLOR_INSET_1)
            border_color_2_disabled: (THEME_COLOR_INSET_1)
            instance border_radius: 4.0
        }

        draw_text: {
            text_style: { font_size: 11.0 }
            color: (THEME_COLOR_TEXT)
            color_hover: (THEME_COLOR_TEXT)
            color_focus: (THEME_COLOR_TEXT)
            color_down: (THEME_COLOR_TEXT)
            color_disabled: (THEME_COLOR_TEXT_DISABLED)
            color_empty: (THEME_COLOR_TEXT_PLACEHOLDER)
            color_empty_hover: (THEME_COLOR_TEXT_PLACEHOLDER_HOVER)
            color_empty_focus: (THEME_COLOR_TEXT)
        }
        draw_selection: { color: (THEME_COLOR_SELECTION) }
    }

    // 行操作按钮 (透明背景 + 无边框)
    pub RowSaveButton = <Button> {
        width: 32.0, height: 32.0
        text: "💾"
        draw_text: {
            color: (THEME_COLOR_VAL)
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: (THEME_COLOR_WHITE)
            color_hover: (THEME_COLOR_LIGHT_HOVER)
            border_radius: 4.0
            border_size: 0.0
        }
    }

    // 删除按钮
    pub RowDeleteButton = <Button> {
        width: 32.0, height: 32.0
        text: "🗑️"
        draw_text: {
            color: (THEME_COLOR_TEXT_META)
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: (THEME_COLOR_WHITE)
            color_hover: (THEME_COLOR_SELECTION_HOVER)
            border_radius: 4.0
            border_size: 0.0
        }
    }

    // 错误提示弹窗
    pub ErrorModal = <Modal> {
        content: <RoundedView> {
            width: 360.0, height: Fit
            spacing: 24.0, padding: 32.0
            flow: Down,
            show_bg: true

            draw_bg: {
                color: (THEME_COLOR_WHITE)
                border_radius: 6.0
            }

            title = <Label> {
                text: "提示"
                draw_text: {
                    color: (THEME_COLOR_WARNING)
                    text_style: { font_size: 18.0 }
                }
            }

            message = <Label> {
                width: Fill
                text: ""
                draw_text: {
                    color: (THEME_COLOR_ERROR)
                    wrap: Word
                    text_style: { font_size: 14.0 }
                }
            }

            <View> {
                width: Fill, height: Fit
                flow: Right, align: {x: 1.0}

                ok_btn = <BlueButton> {
                    width: 100.0, height: 40.0, text: "确定"
                }
            }
        }
    }
}
