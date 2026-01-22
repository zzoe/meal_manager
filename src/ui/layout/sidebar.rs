use crate::ui::app::AppAction;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::components::common::*;

    // 侧边栏定义
    pub Sidebar = {{Sidebar}} {
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
}

#[derive(Live, LiveHook, Widget)]
pub struct Sidebar {
    #[deref]
    view: View,
    #[rust(false)]
    collapsed: bool,
}

impl Widget for Sidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for Sidebar {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // 折叠按钮处理 - 在组件内部处理
        if self.button(id!(btn_toggle)).clicked(actions) {
            self.collapsed = !self.collapsed;

            let width = if self.collapsed { 50.0 } else { 120.0 };
            let btn_text = if self.collapsed { ">>" } else { "<<" };
            let nav_visible = !self.collapsed;

            self.apply_over(cx, live! { width: (width) });
            self.button(id!(btn_toggle)).set_text(cx, btn_text);
            self.view(id!(nav_group)).set_visible(cx, nav_visible);

            self.redraw(cx);
        }

        // 导航按钮处理 - 发送事件给父组件
        let uid = self.widget_uid();
        if self.button(id!(btn_nav_stats)).clicked(actions) {
            cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::NavigateToStats);
        }
        if self.button(id!(btn_nav_config)).clicked(actions) {
            cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::NavigateToConfig);
        }
    }
}
