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
            sidebar = <Sidebar> {}
            content_container = <View> {
                width: Fill, height: Fill
                show_bg: true, draw_bg: { color: (COLOR_BG_APP) }
                navigation = <PageFlip> {
                    width: Fill, height: Fill
                    active_page: stats
                    stats = <StatsPage> {}
                    config = <ConfigPage> {}
                }
            }
        }

        // 使用 Modal
        error_modal = <ErrorModal> {}
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
        let btn = self.button(id!(error_modal.ok_btn));
        if btn.clicked(actions) {
            self.modal(id!(error_modal)).close(cx);
            self.redraw(cx);
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
            inner.view(id!(navigation))
                .as_page_flip()
                .set_active_page(cx, page_id);
        }
    }

    pub fn show_error(&self, cx: &mut Cx, msg: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.label(id!(error_modal.message)).set_text(cx, msg);
            inner.modal(id!(error_modal)).open(cx);
            inner.redraw(cx);
        }
    }
}