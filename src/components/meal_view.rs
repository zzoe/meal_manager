use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;

    // 调色板
    COLOR_BG_APP = #F3F4F6
    COLOR_BG_SIDEBAR = #1F2937
    COLOR_BG_CARD = #FFFFFF

    COLOR_TEXT_PRIMARY = #111827
    COLOR_TEXT_SECONDARY = #6B7280

    // 功能色
    COLOR_PRIMARY = #2196F3
    COLOR_PRIMARY_HOVER = #1976D2

    // 结果卡片头部背景
    COLOR_LUNCH_HEADER = #FFF7ED    // Orange-50
    COLOR_DINNER_HEADER = #EEF2FF   // Indigo-50
    COLOR_ERROR_HEADER = #FEF2F2    // Red-50

    // 结果卡片标题文字
    COLOR_LUNCH_TEXT = #B45309      // Orange-700
    COLOR_DINNER_TEXT = #4338CA     // Indigo-700
    COLOR_ERROR_TEXT = #B91C1C      // Red-700

    COLOR_BORDER = #E5E7EB

    // 1. 结果展示卡片 (美化版)
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

    // 2. 侧边栏导航按钮
    pub NavButton = <Button> {
        width: Fill, height: 40.0
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

            // 确保状态切换时颜色正确
            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
        }
    }

    // 3. 侧边栏折叠按钮 (小正方形)
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

    // 4. 红色主按钮
    pub RedButton = <Button> {
        width: Fill, height: 48.0
        draw_text: {
            color: #fff
            text_style: { font_size: 14.0 }
        }
        draw_bg: {
            // 显式定义所有状态颜色，防止 default/hover/down 之间插值出透明度
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

    // 5. 纯净输入框
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

    pub MealView = {{MealView}} {
        width: Fill, height: Fill
        flow: Right, spacing: 0.0

        // --- 左侧：侧边栏 ---
        sidebar = <View> {
            width: 120.0, height: Fill
            flow: Down, spacing: 10.0, padding: 10.0
            show_bg: true, draw_bg: { color: (COLOR_BG_SIDEBAR) }

            // 顶部折叠按钮区
            <View> {
                width: Fill, height: Fit, align: {x: 1.0}
                btn_toggle = <ToggleButton> { text: "<<" }
            }

            // 导航区
            nav_group = <View> {
                width: Fill, height: Fit
                flow: Down, spacing: 5.0

                btn_nav_stats = <NavButton> { text: "数据统计" }
                btn_nav_config = <NavButton> { text: "人员配置" }
            }
        }

        // --- 右侧：工作区 ---
        content_container = <View> {
            width: Fill, height: Fill
            show_bg: true, draw_bg: { color: (COLOR_BG_APP) }

            // >>>>>> 页面 1: 统计页
            page_stats = <View> {
                visible: true
                width: Fill, height: Fill
                flow: Right, spacing: 15.0, padding: 15.0

                // 列 1：数据输入面板
                <RoundedView> {
                    width: 240.0, height: Fill
                    flow: Down, spacing: 10.0, padding: 15.0

                    draw_bg: {
                        color: (COLOR_BG_CARD)
                        border_color: (COLOR_BORDER)
                        border_size: 1.0
                        border_radius: 8.0
                    }

                    <Label> {
                        text: "数据录入"
                        draw_text: { color: (COLOR_TEXT_PRIMARY), text_style: { font_size: 14.0 } }
                    }

                    <Label> {
                        text: "Shift+Enter 换行"
                        draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 11.0 } }
                    }

                    // 输入框容器
                    <RoundedView> {
                        width: Fill, height: Fill
                        draw_bg: {
                            color: #F9FAFB
                            border_color: (COLOR_BORDER)
                            border_size: 1.0
                            border_radius: 4.0
                        }
                        padding: 8.0

                        <ScrollYView> {
                            width: Fill, height: Fill
                            input_box = <CleanTextInput> {
                                text: ""
                            }
                        }
                    }

                    btn_run = <RedButton> { text: "开始分析" }
                }

                // 列 2：结果仪表盘
                <View> {
                    width: Fill, height: Fill
                    flow: Down, spacing: 10.0

                    // 1. 🍱 中餐 (x份)
                    <View> {
                        width: Fill, height: Fill,
                        lunch_card = <ResultCard> {
                            header_slot = {
                                draw_bg: { color: (COLOR_LUNCH_HEADER) }
                                header_label = { text: "🍱 中餐", draw_text: { color: (COLOR_LUNCH_TEXT) } }
                            }
                        }
                    }

                    // 2. 🥘 晚餐 (y份)
                    <View> {
                        width: Fill, height: Fill,
                        dinner_card = <ResultCard> {
                            header_slot = {
                                draw_bg: { color: (COLOR_DINNER_HEADER) }
                                header_label = { text: "🥘 晚餐", draw_text: { color: (COLOR_DINNER_TEXT) } }
                            }
                        }
                    }

                    // 3. ⚠️ 异常监控 (z条)
                    <View> {
                        width: Fill, height: Fill,
                        exception_card = <ResultCard> {
                            header_slot = {
                                draw_bg: { color: (COLOR_ERROR_HEADER) }
                                header_label = { text: "⚠️ 异常监控", draw_text: { color: (COLOR_ERROR_TEXT) } }
                            }
                        }
                    }
                }
            }

            // >>>>>> 页面 2: 配置页
            page_config = <View> {
                visible: false
                width: Fill, height: Fill
                flow: Down, spacing: 20.0, padding: 40.0

                <View> {
                    width: Fill, height: Fit
                    flow: Down, spacing: 5.0
                    <Label> {
                        text: "员工配置"
                        draw_text: { color: (COLOR_TEXT_PRIMARY), text_style: { font_size: 18.0 } }
                    }
                    <Label> {
                        text: "姓名: 昵称1, 昵称2"
                        draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 12.0 } }
                    }
                }

                <RoundedView> {
                    width: Fill, height: Fill
                    draw_bg: {
                        color: (COLOR_BG_CARD)
                        border_color: (COLOR_BORDER)
                        border_size: 1.0
                        border_radius: 8.0
                    }
                    padding: 10.0

                    <ScrollYView> {
                        width: Fill, height: Fill
                        config_input = <CleanTextInput> {
                            text: "加载中..."
                        }
                    }
                }

                <View> {
                    width: Fill, height: Fit, align: {x: 1.0}
                    btn_save_config = <RedButton> { width: 150.0, text: "保存配置" }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MealUiAction {
    NavToStats,
    NavToConfig,
    ToggleSidebar,
    SubmitText(String),
    SaveConfig(String),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MealView {
    #[deref]
    view: View,
    #[rust(false)]
    sidebar_collapsed: bool, // 侧边栏状态
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

        // 侧边栏折叠
        if self.button(id!(btn_toggle)).clicked(actions) {
            self.sidebar_collapsed = !self.sidebar_collapsed;

            // 动态修改样式
            let width = if self.sidebar_collapsed { 50.0 } else { 120.0 };
            let btn_text = if self.sidebar_collapsed { ">>" } else { "<<" };
            let nav_visible = !self.sidebar_collapsed;

            self.view(id!(sidebar))
                .apply_over(cx, live! { width: (width) });
            self.button(id!(btn_toggle)).set_text(cx, btn_text);
            self.view(id!(nav_group)).set_visible(cx, nav_visible);

            self.redraw(cx);
        }

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
            // 更新标题
            inner
                .label(id!(lunch_card.header_slot.header_label))
                .set_text(cx, lunch_title);
            inner
                .label(id!(dinner_card.header_slot.header_label))
                .set_text(cx, dinner_title);
            inner
                .label(id!(exception_card.header_slot.header_label))
                .set_text(cx, exception_title);

            // 更新内容
            inner
                .label(id!(lunch_card.content_view.content))
                .set_text(cx, lunch_text);
            inner
                .label(id!(dinner_card.content_view.content))
                .set_text(cx, dinner_text);
            inner
                .label(id!(exception_card.content_view.content))
                .set_text(cx, exception_text);

            inner.redraw(cx);
        }
    }

    pub fn reset_loading_status(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.button(id!(btn_run)).set_text(cx, "开始统计");
            inner.redraw(cx);
        }
    }

    // 复用 reset 逻辑来显示加载中
    pub fn set_loading_status(&self, cx: &mut Cx, msg: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.button(id!(btn_run)).set_text(cx, msg);
            inner.redraw(cx);
        }
    }
}
