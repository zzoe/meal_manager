use crate::employees::load_config;
use crate::ui::app::backend_handler::handle_backend_result;
use crate::ui::app::ui_handler::handle_ui_action;
use crate::ui::layout::app_shell::AppShellWidgetRefExt;
use crate::ui::pages::employees::page::ConfigPageAction;
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
                pass: {clear_color: #F9FAFBFF}

                caption_bar = {
                    visible: true,
                    margin: {left: -200},
                    caption_label = {
                        label = {
                            text: "Meal Manager"
                            draw_text: { color: #000000FF }
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

                let config_act = widget_action.cast::<ConfigPageAction>();
                match config_act {
                    ConfigPageAction::ValidationError(msg) => {
                        println!("ConfigPageAction::ValidationError({msg})");
                        self.ui.as_app_shell().show_error(cx, &msg);
                    }
                    _ => (),
                }
            }

            handle_backend_result(cx, actions, &self.ui);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
