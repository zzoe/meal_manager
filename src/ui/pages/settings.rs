//! 设置页面 - UI相关

use makepad_widgets::*;

/// 设置页面状态（UI相关）
pub struct SettingsState {
    pub editing_employee: Option<crate::core::EmployeeData>,
    pub new_employee_name: String,
    pub new_employee_nicknames: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            editing_employee: None,
            new_employee_name: String::new(),
            new_employee_nicknames: String::new(),
        }
    }
}

live_design! {
    use link::widgets::*;

    SettingsPage = <View> {
        flow: Down,
        spacing: 20,

        section_title = <Label> {
            text: "员工管理",
            draw_text: {
                color: #333,
                text_style: <THEME_FONT_BOLD> {
                    font_size: 24.0,
                }
            }
        },

        add_section = <View> {
            flow: Down,
            spacing: 10,

            add_title = <Label> {
                text: "添加员工",
                draw_text: {
                    color: #333,
                    text_style: <THEME_FONT_BOLD> {
                        font_size: 18.0,
                    }
                }
            },

            name_input = <View> {
                flow: Right,
                spacing: 10,
                align: { x: 0.0, y: 0.5 },

                name_label = <Label> {
                    text: "姓名:",
                    draw_text: {
                        color: #666,
                        text_style: {
                            font_size: 14.0,
                        }
                    }
                },

                name_field = <TextInput> {
                    width: 200,
                    height: Fit,
                    draw_bg: {
                        color: #fff,
                    },
                    draw_text: {
                        color: #333,
                    }
                }
            },

            nicknames_input = <View> {
                flow: Right,
                spacing: 10,
                align: { x: 0.0, y: 0.5 },

                nicknames_label = <Label> {
                    text: "昵称(逗号分隔):",
                    draw_text: {
                        color: #666,
                        text_style: {
                            font_size: 14.0,
                        }
                    }
                },

                nicknames_field = <TextInput> {
                    width: 300,
                    height: Fit,
                    draw_bg: {
                        color: #fff,
                    },
                    draw_text: {
                        color: #333,
                    }
                }
            },

            add_button = <Button> {
                width: Fit,
                height: Fit,
                padding: { top: 5, bottom: 5, left: 15, right: 15 },
                text: "添加员工",
                draw_text: {
                    color: #fff,
                },
                draw_bg: {
                    color: #4CAF50,
                    radius: 4.0,
                }
            }
        },

        employees_list = <View> {
            flow: Down,
            spacing: 10,

            list_title = <Label> {
                text: "员工列表",
                draw_text: {
                    color: #333,
                    text_style: <THEME_FONT_BOLD> {
                        font_size: 18.0,
                    }
                }
            },

            list_container = <View> {
                flow: Down,
                spacing: 5,
                draw_bg: {
                    color: #f9f9f9,
                    radius: 4.0,
                },
                padding: 10,

                // 员工列表项将在这里动态添加
                placeholder = <Label> {
                    text: "暂无员工数据",
                    draw_text: {
                        color: #999,
                        text_style: {
                            font_size: 14.0,
                        }
                    }
                }
            }
        }
    }
}

/// 设置页面组件 - UI相关
#[derive(Live, LiveHook)]
pub struct SettingsPage {
    #[live]
    ui: WidgetRef,
    #[rust]
    #[allow(dead_code)]
    state: SettingsState,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self {
            ui: WidgetRef::default(),
            state: SettingsState::default(),
        }
    }
}

impl LiveRegister for SettingsPage {
    fn live_register(cx: &mut Cx) {
        live_design(cx);
    }
}

impl SettingsPage {
    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui.handle_event(cx, event, scope);
    }

    pub fn draw(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        self.ui.draw_all(cx, scope);
    }
}
