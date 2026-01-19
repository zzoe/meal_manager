use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;

    // ============================================
    // COLORS
    // ============================================

    // 应用背景
    pub COLOR_BG_APP = #F3F4F6
    pub COLOR_BG_SIDEBAR = #1F2937
    pub COLOR_BG_CARD = #FFFFFF

    // 文字颜色
    pub COLOR_TEXT_PRIMARY = #111827
    pub COLOR_TEXT_SECONDARY = #6B7280

    // 功能色
    pub COLOR_PRIMARY = #2196F3
    pub COLOR_PRIMARY_HOVER = #1976D2

    // 结果卡片头部背景
    pub COLOR_LUNCH_HEADER = #FFF7ED    // Orange-50
    pub COLOR_DINNER_HEADER = #EEF2FF   // Indigo-50
    pub COLOR_ERROR_HEADER = #FEF2F2    // Red-50

    // 结果卡片标题文字
    pub COLOR_LUNCH_TEXT = #B45309      // Orange-700
    pub COLOR_DINNER_TEXT = #4338CA     // Indigo-700
    pub COLOR_ERROR_TEXT = #B91C1C      // Red-700

    pub COLOR_BORDER = #E5E7EB

    // ============================================
    // REUSABLE WIDGETS
    // ============================================

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

        header_slot = <RoundedYView> {
            width: Fill, height: 40.0
            padding: {top: 10.0, left: 15.0}
            draw_bg: {
                color: #f0f0f0
                border_radius: vec2(8.0, 1.0),
            }

            header_label = <Label> {
                text: "Title"
                draw_text: {
                    text_style: { font_size: 12.0 }
                }
            }
        }

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
        width: Fit, height: 40.0
        draw_text: {
            color: #D1D5DB
            text_style: { font_size: 12.0 }
        }
        draw_bg: {
            color: #0000
            border_size: 0.0
            border_radius: 4.0
            color_hover: #374151
            color_down: #111827

            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
        }
    }

    // 折叠按钮
    pub ToggleButton = <Button> {
        width: Fill, height: 30.0
        draw_text: {
            color: #9CA3AF
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: #0000
            border_size: 0.0
            color_hover: #374151
            color_down: #111827
        }
    }

    // 红色主按钮
    pub RedButton = <Button> {
        width: Fill, height: 48.0
        draw_text: {
            color: #fff
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: #EF4444
            color_hover: #DC2626
            color_down: #B91C1C
            color_focus: #EF4444

            border_size: 0.0
            border_radius: 6.0

            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
        }
    }

    // 纯净输入框
    pub CleanTextInput = <TextInput> {
        width: Fill, height: Fit
        padding: 0.0

        draw_bg: {
            color: #0000
            border_size: 0.0
            instance hover: 0.0
            instance focus: 0.0
            color_hover: #0000
            color_focus: #0000
        }

        draw_text: {
            text_style: { font_size: 13.0 }
            color: #111827
            color_hover: #111827
            color_focus: #111827
        }

        draw_cursor: {
            color: (COLOR_PRIMARY)
        }
        draw_selection: {
            color: #BFDBFE
        }
    }
}
