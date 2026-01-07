//! 餐饮统计页面 - UI相关

use makepad_widgets::*;

/// 餐饮页面状态（UI相关）
pub struct DiningState {
    pub input_text: String,
    pub report: crate::core::Report,
}

impl Default for DiningState {
    fn default() -> Self {
        Self {
            input_text: "示例：\nEliza: 10\n张三: 20".to_string(),
            report: crate::core::Report::default(),
        }
    }
}

impl DiningState {
    /// 分析就餐数据（调用核心业务逻辑）
    pub fn analyze_dining(
        &mut self,
        employees: &[crate::core::EmployeeData],
    ) -> anyhow::Result<()> {
        use crate::core::dining_analysis::DiningAnalyzer;

        let analyzer = DiningAnalyzer::new()?;
        self.report = analyzer.analyze(&self.input_text, employees)?;
        Ok(())
    }
}

live_design! {
    use link::widgets::*;

    DiningPage = <View> {
        flow: Right,
        spacing: 20,

        // 左侧：输入区域
        input_section = <View> {
            flow: Down,
            spacing: 15,
            width: Fill,

            section_title = <Label> {
                text: "报餐数据输入",
                draw_text: {
                    color: #333,
                    text_style: <THEME_FONT_BOLD> {
                        font_size: 18.0,
                    }
                }
            },

            input_label = <Label> {
                text: "请输入报餐数据（每行格式：姓名: 餐数）",
                draw_text: {
                    color: #666,
                    text_style: {
                        font_size: 14.0,
                    }
                }
            },

            text_input = <TextInput> {
                width: Fill,
                height: 200,
                text: "示例：\nEliza: 10\n张三: 20",
                draw_bg: {
                    color: #fff,
                },
                draw_text: {
                    color: #333,
                }
            },

            calculate_button = <Button> {
                width: Fit,
                height: Fit,
                padding: { top: 10, bottom: 10, left: 20, right: 20 },
                text: "计算",
                draw_text: {
                    color: #fff,
                },
                draw_bg: {
                    color: #2196F3,
                    radius: 4.0,
                }
            }
        },

        // 右侧：结果显示区域
        result_section = <View> {
            flow: Down,
            spacing: 15,
            width: Fill,

            section_title = <Label> {
                text: "统计结果",
                draw_text: {
                    color: #333,
                    text_style: <THEME_FONT_BOLD> {
                        font_size: 18.0,
                    }
                }
            },

            // 总计区域
            totals_container = <View> {
                flow: Right,
                spacing: 20,

                lunch_total_card = <View> {
                    flow: Down,
                    spacing: 5,
                    padding: 15,
                    width: Fit,
                    height: Fit,
                    draw_bg: {
                        color: #E3F2FD,
                        radius: 8.0,
                    },

                    lunch_label = <Label> {
                        text: "午餐总数",
                        draw_text: {
                            color: #1976D2,
                            text_style: {
                                font_size: 14.0,
                            }
                        }
                    },

                    lunch_value = <Label> {
                        text: "0",
                        draw_text: {
                            color: #1976D2,
                            text_style: <THEME_FONT_BOLD> {
                                font_size: 24.0,
                            }
                        }
                    }
                },

                dinner_total_card = <View> {
                    flow: Down,
                    spacing: 5,
                    padding: 15,
                    width: Fit,
                    height: Fit,
                    draw_bg: {
                        color: #F1F8E9,
                        radius: 8.0,
                    },

                    dinner_label = <Label> {
                        text: "晚餐总数",
                        draw_text: {
                            color: #689F38,
                            text_style: {
                                font_size: 14.0,
                            }
                        }
                    },

                    dinner_value = <Label> {
                        text: "0",
                        draw_text: {
                            color: #689F38,
                            text_style: <THEME_FONT_BOLD> {
                                font_size: 24.0,
                            }
                        }
                    }
                }
            },

            // 详情区域
            details_container = <View> {
                flow: Down,
                spacing: 10,

                details_label = <Label> {
                    text: "详细统计",
                    draw_text: {
                        color: #333,
                        text_style: <THEME_FONT_BOLD> {
                            font_size: 16.0,
                        }
                    }
                },

                lunch_details_area = <View> {
                    flow: Down,
                    spacing: 5,

                    lunch_details_label = <Label> {
                        text: "午餐详情：",
                        draw_text: {
                            color: #666,
                            text_style: {
                                font_size: 14.0,
                            }
                        }
                    },

                    lunch_details_text = <Label> {
                        text: "暂无数据",
                        draw_text: {
                            color: #666,
                            text_style: {
                                font_size: 13.0,
                            }
                        },
                    }
                },

                dinner_details_area = <View> {
                    flow: Down,
                    spacing: 5,

                    dinner_details_label = <Label> {
                        text: "晚餐详情：",
                        draw_text: {
                            color: #666,
                            text_style: {
                                font_size: 14.0,
                            }
                        }
                    },

                    dinner_details_text = <Label> {
                        text: "暂无数据",
                        draw_text: {
                            color: #666,
                            text_style: {
                                font_size: 13.0,
                            }
                        },
                    }
                },

                missing_area = <View> {
                    flow: Down,
                    spacing: 5,

                    missing_label = <Label> {
                        text: "未报餐人员：",
                        draw_text: {
                            color: #666,
                            text_style: {
                                font_size: 14.0,
                            }
                        }
                    },

                    missing_text = <Label> {
                        text: "无",
                        draw_text: {
                            color: #666,
                            text_style: {
                                font_size: 13.0,
                            }
                        },
                    }
                }
            }
        }
    }
}

/// 餐饮页面组件 - UI相关
#[derive(Live, LiveHook)]
pub struct DiningPage {
    #[live]
    ui: WidgetRef,
    #[rust]
    #[allow(dead_code)]
    state: DiningState,
}

impl LiveRegister for DiningPage {
    fn live_register(cx: &mut Cx) {
        // 注册页面组件的 live_design
        live_design(cx);
    }
}

impl Default for DiningPage {
    fn default() -> Self {
        Self {
            ui: WidgetRef::default(),
            state: DiningState::default(),
        }
    }
}

impl DiningPage {
    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui.handle_event(cx, event, scope);
    }

    pub fn draw(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        self.ui.draw_all(cx, scope);
    }
}
