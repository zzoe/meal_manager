//! UI相关模块 - 包含所有用户界面定义

use makepad_widgets::*;

use crate::core::EmployeeData;

pub mod pages;
pub use pages::{DiningPage, SettingsPage};

/// UI 页面枚举
#[derive(Clone, Default, PartialEq, Debug)]
pub enum CurrentPage {
    #[default]
    Dining,
    Settings,
}

live_design! {
    use link::widgets::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Down,
                    spacing: 20,
                    align: { x: 0.0, y: 0.0 },
                    padding: 20,

                    // 标题区域
                    header = <View> {
                        flow: Down,
                        spacing: 10,

                        title_bar = <View> {
                            flow: Right,
                            spacing: 20,
                            align: { x: 0.0, y: 0.5 },

                            title = <Label> {
                                text: "报餐助手 Pro (Makepad)",
                                draw_text: {
                                    color: #333,
                                    text_style: <THEME_FONT_BOLD> {
                                        font_size: 24.0,
                                    }
                                }
                            },

                            nav_buttons = <View> {
                                flow: Right,
                                spacing: 10,

                                dining_button = <Button> {
                                    width: Fit,
                                    height: Fit,
                                    padding: { top: 5, bottom: 5, left: 15, right: 15 },
                                    text: "餐饮统计",
                                    draw_text: {
                                        color: #fff,
                                    },
                                    draw_bg: {
                                        color: #2196F3,
                                        radius: 4.0,
                                    }
                                },

                                settings_button = <Button> {
                                    width: Fit,
                                    height: Fit,
                                    padding: { top: 5, bottom: 5, left: 15, right: 15 },
                                    text: "设置",
                                    draw_text: {
                                        color: #666,
                                    },
                                    draw_bg: {
                                        color: #f5f5f5,
                                        radius: 4.0,
                                    }
                                }
                            }
                        },

                        status_label = <Label> {
                            text: "就绪",
                            draw_text: {
                                color: #666,
                                text_style: {
                                    font_size: 14.0,
                                }
                            }
                        }
                    },

                    // 主要内容区域 - 动态切换页面
                    content_area = <View> {
                        width: Fill,
                        height: Fill,

                        // 页面内容将在这里动态切换
                        dining_page = <View> {},
                        settings_page = <View> {},
                    }
                }
            }
        }
    }
}

/// 主应用状态
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    current_page: CurrentPage,
    #[rust]
    #[allow(dead_code)]
    employees: Vec<EmployeeData>,
    #[rust]
    #[allow(dead_code)]
    status: String,
    #[rust]
    dining_page: DiningPage,
    #[rust]
    settings_page: SettingsPage,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ui: WidgetRef::default(),
            current_page: CurrentPage::Dining,
            employees: Vec::new(),
            status: "就绪".to_string(),
            dining_page: DiningPage::default(),
            settings_page: SettingsPage::default(),
        }
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.tick();

        // 处理页面切换
        if let Event::Actions(actions) = event {
            for action in actions {
                if action.downcast_ref::<ButtonAction>().is_some() {
                    if self.ui.button(id!(dining_button)).pressed(actions) {
                        self.current_page = CurrentPage::Dining;
                    } else if self.ui.button(id!(settings_button)).pressed(actions) {
                        self.current_page = CurrentPage::Settings;
                    }
                }
            }
        }

        // 根据当前页面处理事件
        match self.current_page {
            CurrentPage::Dining => {
                self.dining_page
                    .handle_event(cx, event, &mut Scope::empty());
            }
            CurrentPage::Settings => {
                self.settings_page
                    .handle_event(cx, event, &mut Scope::empty());
            }
        }

        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    pub fn tick(&mut self) {
        // 应用状态更新逻辑
    }

    pub fn switch_page(&mut self, page: CurrentPage) {
        self.current_page = page;
    }

    pub fn draw(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        // 根据当前页面显示对应的UI
        match self.current_page {
            CurrentPage::Dining => {
                self.dining_page.draw(cx, scope);
            }
            CurrentPage::Settings => {
                self.settings_page.draw(cx, scope);
            }
        }

        self.ui.draw_all(cx, scope);
    }
}

app_main!(App);
