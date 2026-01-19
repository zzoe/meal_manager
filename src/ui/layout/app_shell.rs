use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::components::common::*;
    use crate::ui::layout::sidebar::*;
    use crate::ui::pages::stats_page::*;
    use crate::ui::pages::config_page::*;

    pub AppShell = {{AppShell}} {
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
}

#[derive(Clone, Debug, DefaultNone)]
pub enum AppAction {
    NavigateToStats,
    NavigateToConfig,
    SubmitAnalysis(String),
    SaveConfig(String),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct AppShell {
    #[deref]
    view: View,
}

impl Widget for AppShell {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
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
            inner.view(id!(navigation)).as_page_flip().set_active_page(cx, page_id);
        }
    }
}
