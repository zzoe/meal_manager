use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    // 调色板
    pub COLOR_BG_APP = #F3F4F6FF
    pub COLOR_BG_SIDEBAR = #1F2937FF
    pub COLOR_BG_CARD = #FFFFFFFF

    pub COLOR_TEXT_PRIMARY = #111827FF
    pub COLOR_TEXT_SECONDARY = #6B7280FF

    // 功能色
    pub COLOR_PRIMARY = #2196F3FF

    // 结果卡片头部背景
    pub COLOR_LUNCH_HEADER = #FFF7EDFF    // Orange-50
    pub COLOR_DINNER_HEADER = #EEF2FFFF   // Indigo-50
    pub COLOR_ERROR_HEADER = #FEF2F2FF    // Red-50

    // 结果卡片标题文字
    pub COLOR_LUNCH_TEXT = #B45309FF      // Orange-700
    pub COLOR_DINNER_TEXT = #4338CAFF     // Indigo-700
    pub COLOR_ERROR_TEXT = #B91C1CFF      // Red-700

    pub COLOR_BORDER = #E5E7EBFF

    // 结果展示卡片
    pub ResultCard = <RoundedView> {
        width: Fill, height: Fill
        flow: Down, spacing: 0.0

        draw_bg: {
            color: (COLOR_BG_CARD)
            border_color: (COLOR_BORDER)
            border_size: 1.0
            border_radius: 8.0
        }

        // 标题栏 (带背景色)
        header_slot = <RoundedYView> {
            width: Fill, height: 40.0
            padding: {top: 10.0, left: 15.0}
            draw_bg: {
                color: #F0F0F0FF
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
                    color: (COLOR_TEXT_PRIMARY)
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
            color: #D1D5DBFF
            text_style: { font_size: 12.0 }
        }
        draw_bg: {
            color: #00000000
            border_size: 0.0
            border_radius: 4.0
            color_hover: #374151FF
            color_down: #111827FF

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
            color: #9CA3AFFF
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: #00000000
            border_size: 0.0
            color_hover: #374151FF
            color_down: #111827FF
        }
    }

    // 蓝色按钮
    pub BlueButton = <Button> {
        width: Fill, height: 48.0
        draw_text: {
            color: #FFFFFF
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: #2196F3FF
            color_hover: #1976D2FF
            color_down: #1565C0FF
            color_focus: #2196F3FF

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
            color: #F9FAFBFF
            instance border_radius: 4.0
        }

        draw_text: {
            text_style: { font_size: 11.0 }
            color: (COLOR_TEXT_PRIMARY)
        }
        draw_selection: { color: #BFDBFEFF }
    }

    // 行操作按钮 (透明背景 + 无边框)
    pub RowSaveButton = <Button> {
        width: 32.0, height: 32.0
        text: "💾"
        draw_text: {
            color: (COLOR_PRIMARY)
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: #FFFFFFFF
            color_hover: #E5E7EBFF
            border_radius: 4.0
            border_size: 0.0
        }
    }

    // 删除按钮
    pub RowDeleteButton = <Button> {
        width: 32.0, height: 32.0
        text: "🗑️"
        draw_text: {
            color: (COLOR_TEXT_SECONDARY)
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: #FFFFFFFF
            color_hover: #FEE2E2FF
            border_radius: 4.0
            border_size: 0.0
        }
    }

    // 错误提示弹窗 - 使用标准对齐方案确保文本可见
    pub ErrorModal = <Modal> {
        width: Fill, height: Fill

        content = <View> {
            width: Fill, height: Fill
            show_bg: true, draw_bg: { color: #00000000 }
            // 使用对齐而不是坐标偏移，确保渲染稳定
            align: {x: 0.5, y: 0.5}

            inner_content = <RoundedView> {
                width: 320.0, height: Fit
                flow: Down, spacing: 20.0, padding: 25.0

                draw_bg: {
                    color: #FFFFFFFF
                    border_radius: 12.0
                    border_color: (COLOR_BORDER)
                    border_size: 1.0
                }

                title = <Label> {
                    text: "提示"
                    draw_text: {
                        color: (COLOR_TEXT_PRIMARY)
                        text_style: { font_size: 14.0, font_weight: 700 }
                    }
                }

                message = <Label> {
                    width: Fill
                    text: ""
                    draw_text: {
                        color: (COLOR_TEXT_SECONDARY)
                        wrap: Word
                        text_style: { font_size: 12.0 }
                    }
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Right, align: {x: 1.0}
                    ok_btn = <BlueButton> {
                        width: 100.0, height: 38.0, text: "确定"
                    }
                }
            }
        }
    }
}
