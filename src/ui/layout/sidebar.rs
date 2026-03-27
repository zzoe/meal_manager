use crate::ui::application::AppAction;
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

            btn_nav_stats = <NavButton> {
                text: "📊 报餐统计"
                // 展开状态下的样式
                draw_text: { text_style: { font_size: 12.0 } }
            }
            btn_nav_employees = <NavButton> {
                text: "👥 员工登记"
                draw_text: { text_style: { font_size: 12.0 } }
            }
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

            // 切换导航按钮的显示模式
            if self.collapsed {
                // 收缩状态：只显示图标，调整样式
                self.button(id!(btn_nav_stats)).set_text(cx, "📊");
                self.button(id!(btn_nav_employees)).set_text(cx, "👥");
                // 调整按钮样式为图标模式
                self.button(id!(btn_nav_stats)).apply_over(
                    cx,
                    live! {
                        draw_text: { text_style: { font_size: 12.0 } }
                    },
                );
                self.button(id!(btn_nav_employees)).apply_over(
                    cx,
                    live! {
                        draw_text: { text_style: { font_size: 12.0 } }
                    },
                );
            } else {
                // 展开状态：显示图标+文字
                self.button(id!(btn_nav_stats)).set_text(cx, "📊 报餐统计");
                self.button(id!(btn_nav_employees))
                    .set_text(cx, "👥 员工登记");
                // 恢复按钮样式为文字模式
                self.button(id!(btn_nav_stats)).apply_over(
                    cx,
                    live! {
                        draw_text: { text_style: { font_size: 12.0 } }
                    },
                );
                self.button(id!(btn_nav_employees)).apply_over(
                    cx,
                    live! {
                        draw_text: { text_style: { font_size: 12.0 } }
                    },
                );
            }

            self.redraw(cx);
        }

        // 导航按钮处理
        let uid = self.widget_uid();
        if self.button(id!(btn_nav_stats)).clicked(actions) {
            cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::NavigateToStats);
        }
        if self.button(id!(btn_nav_employees)).clicked(actions) {
            cx.widget_action(
                uid,
                &HeapLiveIdPath::default(),
                AppAction::NavigateToEmployees,
            );
        }
    }
}
