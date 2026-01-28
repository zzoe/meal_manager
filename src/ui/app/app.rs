use crate::app::{handle_backend_result, handle_ui_action};
use crate::services::{BackendResult, load_config};
use crate::ui::layout::app_shell::AppShellWidgetRefExt;
use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum AppAction {
    NavigateToStats,
    NavigateToConfig,
    None,
}

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::layout::app_shell::AppShell;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {inner_size: vec2(1000, 600)},
                pass: {clear_color: #F2E6D8}

                caption_bar = {
                    visible: true,
                    margin: {left: -200},
                    caption_label = {
                        label = {
                            text: "Meal Manager"
                            draw_text: { color: #000 }
                        }
                    }
                },

                body = <AppShell> {}
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust(vec!["stats".to_string(),"config".to_string()])]
    precompile_queue: Vec<String>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::ui::register_live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // 应用启动时异步加载配置
        cx.spawn_thread(load_config);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let Some(widget_action) = action.as_widget_action() {
                let act = widget_action.cast::<AppAction>();
                match act {
                    AppAction::None => (),
                    _ => handle_ui_action(cx, &act, &self.ui),
                }
            }

            let result = makepad_widgets::ActionCast::<BackendResult>::cast(action);
            match result {
                BackendResult::None => (),
                _ => handle_backend_result(cx, &result, &self.ui),
            }
        }
    }

    fn handle_next_frame(&mut self, cx: &mut Cx, _e: &NextFrameEvent) {
        if let Some(page) = self.precompile_queue.pop() {
            // 切换到该页面以触发GPU编译
            println!("开始预编译页面: {}", page);
            let app_shell = self
                .ui
                .widget(&[LiveId::from_str("main_window"), LiveId::from_str("body")])
                .as_app_shell();
            app_shell.show_page(cx, &page);
            cx.new_next_frame();
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
