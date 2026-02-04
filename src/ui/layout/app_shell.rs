use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::components::common::*;
    use crate::ui::layout::sidebar::Sidebar;
    use crate::ui::pages::meal_stats::StatsPage;
    use crate::ui::pages::employees::page::ConfigPage;

    pub AppShell = {{AppShell}} {
        width: Fill, height: Fill
        flow: Overlay

        main_view = <View> {
            width: Fill, height: Fill
            flow: Right, spacing: 0.0

            // --- 左侧：侧边栏 ---
            sidebar = <Sidebar> {}

            // --- 右侧：工作区 ---
            content_container = <View> {
                width: Fill, height: Fill
                show_bg: true, draw_bg: { color: (COLOR_BG_APP) }

                // 使用 PageFlip 进行页面切换
                navigation = <PageFlip> {
                    width: Fill, height: Fill
                    active_page: stats

                    // 页面 1: 统计页
                    stats = <StatsPage> {}

                    // 页面 2: 配置页
                    config = <ConfigPage> {}
                }
            }
        }

        // 使用绝对定位的 View 包裹各种弹窗
        modal_view = <View> {
            abs_pos: vec2(0, 0)
            width: Fill, height: Fill
            error_modal = <ErrorModal> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct AppShell {
    #[deref]
    view: View,
}

impl Widget for AppShell {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for AppShell {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let btn = self.button(id!(modal_view.error_modal.content.inner_content.ok_btn));
        if btn.clicked(actions) {
            println!("OK button clicked, closing modal");
            self.modal(id!(modal_view.error_modal)).close(cx);
        }
    }
}

impl AppShellRef {
    pub fn show_page(&self, cx: &mut Cx, page: &str) {
        if let Some(inner) = self.borrow() {
            let page_id = match page {
                "stats" => live_id!(stats),
                "config" => live_id!(config),
                _ => live_id!(stats),
            };
            inner
                .view(id!(navigation))
                .as_page_flip()
                .set_active_page(cx, page_id);
        }
    }

    pub fn show_error(&self, cx: &mut Cx, msg: &str) {
        println!("show_error called: {}", msg);
        match self.borrow() {
            Some(inner) => {
                println!("Successfully borrowed AppShell");
                let modal_view = inner.view(id!(modal_view));
                println!("modal_view obtained: {:?}", modal_view);
                modal_view.set_visible(cx, true);
                println!("modal_view visibility set to true");
                
                let modal = inner.modal(id!(modal_view.error_modal));
                println!("modal obtained: {:?}", modal);
                modal
                    .label(id!(content.inner_content.message))
                    .set_text(cx, msg);
                println!("Message set to: {}", msg);
                println!("Before open, is_open: {}", modal.is_open());
                modal.open(cx);
                println!("After open, is_open: {}", modal.is_open());
                // 强制重绘
                cx.redraw_all();
                println!("Redraw_all called");
            }
            None => {
                println!("ERROR: Failed to borrow AppShell - widget might not be initialized yet");
            }
        }
    }
}
