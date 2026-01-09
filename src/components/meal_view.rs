use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;

    COLOR_BG_PAGE = #F2E6D8
    COLOR_BORDER_RED = #FF6B6B
    COLOR_BG_CARD = #F9F0E6
    COLOR_TEXT_RED = #D00
    COLOR_BTN_NORMAL = #FF6B6B
    COLOR_BTN_HOVER = #FF8888

    // 通用卡片容器
    pub InfoCard = <RoundedView> {
        width: Fill, height: Fill
        flow: Down, spacing: 0.0, padding: 5.0
        draw_bg: {
            color: (COLOR_BG_CARD)
            border_color: (COLOR_BORDER_RED)
            border_size: 2.0
            border_radius: 4.0
        }
        content_view = <ScrollYView> {
            width: Fill, height: Fill
            padding: 5.0
            content = <Label> {
                width: Fill, height: Fit
                text: ""
                draw_text: {
                    color: (COLOR_TEXT_RED)
                    wrap: Word
                    text_style: { font_size: 13.0 }
                }
            }
        }
    }

    pub CleanTextInput = <TextInput> {
        width: Fill, height: Fit
        padding: 0.0

        draw_bg: {
            color: #0000 // 基础透明
            border_size: 0.0

            instance hover: 0.0
            instance focus: 0.0
            color_hover: #0000
            color_focus: #0000
        }

        draw_text: {
            text_style: { font_size: 14.0 }
            color: #333

            // 锁定文字颜色
            color_hover: #333
            color_focus: #333
        }

        draw_cursor: {
            instance focus: 0.0
            color: #333
        }
        draw_selection: {
            color: #FF6B6B44
        }
    }

    // 红色按钮 (无边框)
    pub RedButton = <Button> {
        width: Fill, height: 50.0
        draw_text: {
            color: #fff
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            color: (COLOR_BTN_NORMAL)
            color_hover: (COLOR_BTN_HOVER)
            color_focus: (COLOR_BTN_NORMAL)
            color_down: #D04040
            border_size: 0.0
            border_radius: 4.0
        }
    }

    pub MealView = {{MealView}} {
        width: Fill, height: Fill
        flow: Right, spacing: 0.0

        // --- 左侧：导航栏 ---
        <View> {
            width: 80.0, height: Fill
            flow: Down, spacing: 10.0, padding: {top: 20.0, left: 5.0, right: 5.0}
            show_bg: true, draw_bg: { color: #333 }

            btn_nav_stats = <RedButton> { text: "统计" }
            btn_nav_config = <RedButton> { text: "配置" }
        }

        // --- 右侧：内容区 ---
        content_container = <View> {
            width: Fill, height: Fill
            show_bg: true, draw_bg: { color: (COLOR_BG_PAGE) }

            // === 页面 1: 统计页 ===
            page_stats = <View> {
                visible: true
                width: Fill, height: Fill
                flow: Right, spacing: 0.0

                // 列1：输入区
                <View> {
                    width: Fit, height: Fill
                    flow: Down, spacing: 10.0, padding: 15.0

                    <Label> {
                        text: "今日接龙数据 (Shift+Enter换行):"
                        draw_text: { color: (COLOR_TEXT_RED), text_style: { font_size: 12.0 } }
                    }

                    <RoundedView> {
                        width: Fill, height: Fill
                        draw_bg: {
                            color: #fff8f0
                            border_color: (COLOR_BORDER_RED)
                            border_size: 1.0
                            border_radius: 2.0
                        }
                        padding: 5.0

                        <ScrollYView> {
                            width: Fill, height: Fill
                            input_box = <CleanTextInput> {
                                text: "张三: 01\n李四: 10"
                            }
                        }
                    }

                    btn_run = <RedButton> { text: "开始统计" }
                }

                // 列2：结果展示区
                <View> {
                    width: Fill, height: Fill
                    flow: Down, spacing: 10.0, padding: {top: 20.0, right: 20.0, bottom: 20.0}

                    // 1. 中餐
                    <View> { width: Fill, height: Fill, flow: Down, spacing: 5.0
                        lunch_header = <Label> {
                            text: "中餐:",
                            draw_text: { color: (COLOR_TEXT_RED), text_style: { font_size: 12.0 } }
                        }
                        lunch_card = <InfoCard> {}
                    }

                    // 2. 晚餐
                    <View> { width: Fill, height: Fill, flow: Down, spacing: 5.0
                        dinner_header = <Label> {
                            text: "晚餐:",
                            draw_text: { color: (COLOR_TEXT_RED), text_style: { font_size: 12.0 } }
                        }
                        dinner_card = <InfoCard> {}
                    }

                    // 3. 异常/未报
                    <View> { width: Fill, height: Fill, flow: Down, spacing: 5.0
                        exception_header = <Label> {
                            text: "异常/未报:",
                            draw_text: { color: (COLOR_TEXT_RED), text_style: { font_size: 12.0 } }
                        }
                        exception_card = <InfoCard> {}
                    }
                }
            }

            // === 页面 2: 配置页 ===
            page_config = <View> {
                visible: false
                width: Fill, height: Fill
                flow: Down, spacing: 10.0, padding: 20.0

                <Label> {
                    text: "员工名单 (姓名: 别名1, 别名2)"
                    draw_text: { color: (COLOR_TEXT_RED), text_style: { font_size: 16.0 } }
                }

                <RoundedView> {
                    width: Fill, height: Fill
                    draw_bg: {
                        color: #fff8f0
                        border_color: (COLOR_BORDER_RED)
                        border_size: 1.0
                        border_radius: 2.0
                    }
                    padding: 5.0

                    <ScrollYView> {
                        width: Fill, height: Fill
                        config_input = <CleanTextInput> {
                            text: "加载中..."
                        }
                    }
                }

                btn_save_config = <RedButton> { text: "保存配置" }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MealUiAction {
    NavToStats,
    NavToConfig,
    SubmitText(String),
    SaveConfig(String),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MealView {
    #[deref]
    view: View,
}

impl Widget for MealView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for MealView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.button(id!(btn_nav_stats)).clicked(actions) {
            cx.widget_action(uid, &HeapLiveIdPath::default(), MealUiAction::NavToStats);
        }
        if self.button(id!(btn_nav_config)).clicked(actions) {
            cx.widget_action(uid, &HeapLiveIdPath::default(), MealUiAction::NavToConfig);
        }

        if self.button(id!(btn_run)).clicked(actions) {
            let text = self.text_input(id!(input_box)).text();
            cx.widget_action(
                uid,
                &HeapLiveIdPath::default(),
                MealUiAction::SubmitText(text),
            );
        }
        if self.button(id!(btn_save_config)).clicked(actions) {
            let text = self.text_input(id!(config_input)).text();
            cx.widget_action(
                uid,
                &HeapLiveIdPath::default(),
                MealUiAction::SaveConfig(text),
            );
        }
    }
}

impl MealViewRef {
    pub fn show_page(&self, cx: &mut Cx, page: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            let is_stats = page == "stats";
            inner.view(id!(page_stats)).set_visible(cx, is_stats);
            inner.view(id!(page_config)).set_visible(cx, !is_stats);
            inner.redraw(cx);
        }
    }

    pub fn set_config_text(&self, cx: &mut Cx, v: &str) {
        if let Some(inner) = self.borrow() {
            let input = inner.text_input(id!(config_input));
            input.set_text(cx, v);
            input.redraw(cx);
        }
    }

    pub fn update_results(
        &self,
        cx: &mut Cx,
        lunch_title: &str,
        lunch_text: &str,
        dinner_title: &str,
        dinner_text: &str,
        exception_title: &str,
        exception_text: &str,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.label(id!(lunch_header)).set_text(cx, lunch_title);
            inner
                .label(id!(lunch_card.content_view.content))
                .set_text(cx, lunch_text);

            inner.label(id!(dinner_header)).set_text(cx, dinner_title);
            inner
                .label(id!(dinner_card.content_view.content))
                .set_text(cx, dinner_text);

            inner
                .label(id!(exception_header))
                .set_text(cx, exception_title);
            inner
                .label(id!(exception_card.content_view.content))
                .set_text(cx, exception_text);

            inner.redraw(cx);
        }
    }

    pub fn set_loading_status(&self, cx: &mut Cx, _msg: &str) {
        // 由于 summary_label 移除了，这里暂时可以留空，或者在按钮上显示 loading
        if let Some(mut inner) = self.borrow_mut() {
            inner.button(id!(btn_run)).set_text(cx, "计算中...");
            inner.redraw(cx);
        }
    }

    pub fn reset_loading_status(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.button(id!(btn_run)).set_text(cx, "开始统计");
            inner.redraw(cx);
        }
    }
}
