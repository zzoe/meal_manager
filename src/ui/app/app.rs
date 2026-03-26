use crate::employees::load_config;
use crate::ui::app::backend_handler::handle_backend_result;
use crate::ui::app::ui_handler::handle_ui_action;
use crate::ui::layout::app_shell::AppShellWidgetRefExt;
use crate::ui::pages::employees::employee_page::EmployeePageAction;
use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum AppAction {
    NavigateToStats,
    NavigateToEmployees,
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
                // 初始大小会在 handle_draw 中动态计算并设置
                // 这里使用一个较小的默认值，避免窗口创建时过大
                window: {inner_size: vec2(1000, 618)},
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
        // Windows 平台特定设置
        #[cfg(target_os = "windows")]
        {
            use crate::ui::app::platform;
            use makepad_widgets::makepad_platform::WindowId;
            
            platform::show_caption_buttons(&self.ui, cx);

            // 计算窗口位置和大小
            let (position, size) = platform::calc_window_position();

            log!(
                "Window startup: calculated pos=({:.1},{:.1}), size={:.1}x{:.1}",
                position.x,
                position.y,
                size.x,
                size.y
            );

            // 关键：在窗口创建前修改 create_inner_size 和 create_position
            // WindowHandle::new() 已经将 CreateWindow 操作加入队列，但还未处理
            // 我们修改 cx.windows 中的值，让 CreateWindow 使用正确的参数
            // 注意：当前实现假设应用只有一个主窗口（WindowId 0）
            let window_id = WindowId(0, 0);
            if cx.windows.is_valid(window_id) {
                let window = &mut cx.windows[window_id];
                window.create_inner_size = Some(size);
                window.create_position = Some(position);
                log!(
                    "Window create params updated: size={:.1}x{:.1}, pos={:.1},{:.1}",
                    size.x,
                    size.y,
                    position.x,
                    position.y
                );
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        cx.spawn_thread(load_config);
        #[cfg(target_arch = "wasm32")]
        {
            let _ = cx;
            load_config();
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let Some(widget_action) = action.as_widget_action() {
                // 窗口大小和位置已在 handle_startup 中设置
                // 这里只需要处理其他 action

                let act = widget_action.cast::<AppAction>();
                match act {
                    AppAction::None => (),
                    _ => handle_ui_action(cx, &act, &self.ui),
                }

                let config_act = widget_action.cast::<EmployeePageAction>();
                if let EmployeePageAction::ValidationError(msg) = config_act {
                    println!("EmployeePageAction::ValidationError({msg})");
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
