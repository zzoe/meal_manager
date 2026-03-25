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
    SidebarToggled(f64),
    None,
}

live_design! {
    use link::shaders::*;
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::layout::app_shell::AppShell;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {inner_size: vec2(1000, 600)},
                pass: {clear_color: (THEME_COLOR_BG_APP)}

                caption_bar = {
                    visible: true,
                    draw_bg: { color: (THEME_COLOR_APP_CAPTION_BAR) }
                    caption_label = {
                        label = {
                            text: "Meal Manager"
                            draw_text: { color: (THEME_COLOR_TEXT) }
                        }
                    }

                    windows_buttons = {
                        visible: false,
                        min = <DesktopButton> {
                            draw_bg: {
                                button_type: WindowsMin
                                color: (THEME_COLOR_CAPTION_BUTTON_ICON)
                                color_hover: (THEME_COLOR_CAPTION_BUTTON_HOVER)
                                color_down: (THEME_COLOR_CAPTION_BUTTON_PRESSED)
                            }
                        }
                        max = <DesktopButton> {
                            draw_bg: {
                                button_type: WindowsMax
                                color: (THEME_COLOR_CAPTION_BUTTON_ICON)
                                color_hover: (THEME_COLOR_CAPTION_BUTTON_HOVER)
                                color_down: (THEME_COLOR_CAPTION_BUTTON_PRESSED)
                            }
                        }
                        close = <DesktopButton> {
                            draw_bg: {
                                button_type: WindowsClose
                                color: (THEME_COLOR_CAPTION_BUTTON_ICON)
                                color_hover: (THEME_COLOR_CAPTION_BUTTON_CLOSE_HOVER)
                                color_down: (THEME_COLOR_CAPTION_BUTTON_CLOSE_PRESSED)
                            }
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
        crate::ui::theme_custom_light::live_design(cx);
        cx.link(live_id!(theme), live_id!(theme_custom_light));
        crate::ui::register_live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // 仅在 Windows 平台显示标题栏按钮
        #[cfg(target_os = "windows")]
        {
            self.ui.widget(&[LiveId::from_str("main_window"), LiveId::from_str("caption_bar"), LiveId::from_str("windows_buttons")])
                .set_visible(cx, true);
        }

        #[cfg(not(target_arch = "wasm32"))]
        cx.spawn_thread(load_config);
        #[cfg(target_arch = "wasm32")]
        let _ = cx;
        #[cfg(target_arch = "wasm32")]
        load_config();
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
                if let ConfigPageAction::ValidationError(msg) = config_act {
                    println!("ConfigPageAction::ValidationError({msg})");
                    self.ui
                        .widget(&[LiveId::from_str("body")])
                        .as_app_shell()
                        .show_error(cx, &msg);
                }
            }
        }

        handle_backend_result(cx, actions, &self.ui);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
