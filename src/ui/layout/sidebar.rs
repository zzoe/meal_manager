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
        show_bg: true, draw_bg: { color: (THEME_COLOR_U_5) }

        // 顶部折叠按钮区
        <View> {
            width: Fill, height: Fit, align: {x: 1.0}
            btn_toggle = <ToggleButton> { text: "<<" }
        }

        // 导航区
        nav_group = <View> {
            width: Fill, height: Fit
            flow: Down, spacing: 5.0

            btn_nav_stats = <NavButton> { text: "报餐统计" }
            btn_nav_config = <NavButton> { text: "员工登记" }
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
        // 折叠按钮处理
        if self.button(id!(btn_toggle)).clicked(actions) {
            self.collapsed = !self.collapsed;

            let width = if self.collapsed { 50.0 } else { 120.0 };
            let btn_text = if self.collapsed { ">>" } else { "<<" };

            self.apply_over(cx, live! { width: (width) });
            self.button(id!(btn_toggle)).set_text(cx, btn_text);

            // 折叠时隐藏导航文字，展开时恢复
            // 不使用 set_visible —— 在 WASM/WebGL 环境下，set_visible(false) 会
            // 导致子 widget 的 GPU instance 缓冲区被释放，再次 set_visible(true)
            // 后 animator 的 instance 变量（hover/down/focus）未正确写回，
            // 使 draw_text.get_color() 返回黑色/透明，表现为"灰色方块无文字"。
            // 改用 apply_over 控制 nav_group 高度，让按钮始终参与绘制流程。
            if self.collapsed {
                self.view(id!(nav_group)).apply_over(cx, live! {
                    height: 0.0, margin: 0.0, spacing: 0.0
                });
            } else {
                self.view(id!(nav_group)).apply_over(cx, live! {
                    height: Fit, spacing: 5.0
                });
            }

            self.redraw(cx);
        }

        // 导航按钮处理
        let uid = self.widget_uid();
        if self.button(id!(btn_nav_stats)).clicked(actions) {
            cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::NavigateToStats);
        }
        if self.button(id!(btn_nav_config)).clicked(actions) {
            cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::NavigateToConfig);
        }
    }
}
