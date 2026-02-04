use crate::features::meal_stats::MealAnalysisAction;
use crate::meal_stats::analyzer;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::components::common::*;

    // 统计页面
    pub StatsPage = {{StatsPage}} {
        width: Fill, height: Fill
        flow: Right, spacing: 15.0, padding: 15.0

        // 列 1：数据输入面板
        <RoundedView> {
            width: 240.0, height: Fill
            flow: Down, spacing: 10.0, padding: 15.0

            draw_bg: {
                color: (COLOR_BG_CARD)
                border_color: (COLOR_BORDER)
                border_size: 0.0
                border_radius: 8.0
            }

            <Label> {
                text: "输入点餐数据"
                draw_text: { color: (COLOR_TEXT_PRIMARY), text_style: { font_size: 14.0 } }
            }

            // 输入区域容器：固定边框和底色
            <RoundedView> {
                width: Fill, height: Fill
                flow: Down, margin: { top: 10.0 }
                padding: 10.0
                draw_bg: {
                    color: (COLOR_BG_CARD)
                    border_color: (COLOR_BORDER)
                    border_size: 1.0
                    border_radius: 8.0
                }

                <ScrollYView> {
                    width: Fill, height: Fill

                    input = <TextInput> {
                        width: Fill, height: Fit
                        padding: 15.0
                        empty_text: "此处粘贴点餐内容，例如:\nzoe: 11\n小明: 01..."

                        // 背景交给外层 input_container，保持输入区透明以免覆盖边框
                        draw_bg: {
                            color: #FFFFFFFF
                            color_empty: #FFFFFFFF
                            color_focus: #FFFFFFFF
                            color_hover: #FFFFFFFF
                            color_down: #FFFFFFFF
                            border_size: 0.0
                            border_radius: 4.0
                        }

                        draw_text: {
                            text_style: { font_size: 13.0, line_spacing: 1.6 }
                            color: (COLOR_TEXT_PRIMARY)
                            color_empty: (COLOR_TEXT_PRIMARY)
                            color_focus: (COLOR_TEXT_PRIMARY)
                            color_hover: (COLOR_TEXT_PRIMARY)
                            wrap: Word
                        }

                        draw_selection: {
                            color: #BFDBFEFF
                            color_focus: #BFDBFEFF
                            color_hover: #BFDBFEFF
                        }

                        draw_cursor: {
                            color: #1E40AFFF
                        }
                    }
                }
            }

            btn_run = <BlueButton> {
                width: Fill, text: "开始分析"
            }
        }

        // 列 2：结果展示面板
        <View> {
            width: Fill, height: Fill
            flow: Down, spacing: 15.0

            lunch_card = <ResultCard> {
                header_slot = {
                    draw_bg: { color: (COLOR_LUNCH_HEADER) }
                    header_label = { text: "🍱 中餐", draw_text: { color: (COLOR_LUNCH_TEXT) } }
                }
            }

            dinner_card = <ResultCard> {
                header_slot = {
                    draw_bg: { color: (COLOR_DINNER_HEADER) }
                    header_label = { text: "🥘 晚餐", draw_text: { color: (COLOR_DINNER_TEXT) } }
                }
            }

            exception_card = <ResultCard> {
                header_slot = {
                    draw_bg: { color: (COLOR_ERROR_HEADER) }
                    header_label = { text: "⚠️ 异常与未报", draw_text: { color: (COLOR_ERROR_TEXT) } }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct StatsPage {
    #[deref]
    view: View,
}

impl Widget for StatsPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for StatsPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(id!(btn_run)).clicked(actions) {
            // 更新按钮文本为加载状态
            self.button(id!(btn_run)).set_text(cx, "正在计算...");
            self.button(id!(btn_run)).set_disabled(cx, true);
            self.button(id!(btn_run)).redraw(cx);

            // 获取输入文本并在后台线程执行分析
            let text = self.text_input(id!(input)).text();
            cx.spawn_thread(move || Cx::post_action(analyzer::analyze(text)));
        }
    }
}

impl StatsPageRef {
    pub fn update_results(&self, cx: &mut Cx, action: &MealAnalysisAction) {
        if let MealAnalysisAction::AnalysisComplete {
            lunch_summary,
            lunch_details,
            dinner_summary,
            dinner_details,
            exception_summary,
            exception_details,
        } = action
            && let Some(mut inner) = self.borrow_mut()
        {
            inner
                .label(id!(lunch_card.header_slot.header_label))
                .set_text(cx, lunch_summary);
            inner
                .label(id!(dinner_card.header_slot.header_label))
                .set_text(cx, dinner_summary);
            inner
                .label(id!(exception_card.header_slot.header_label))
                .set_text(cx, exception_summary);

            inner
                .label(id!(lunch_card.content_view.content))
                .set_text(cx, lunch_details);
            inner
                .label(id!(dinner_card.content_view.content))
                .set_text(cx, dinner_details);
            inner
                .label(id!(exception_card.content_view.content))
                .set_text(cx, exception_details);

            inner.button(id!(btn_run)).set_text(cx, "开始分析");
            inner.button(id!(btn_run)).set_disabled(cx, false);

            inner.redraw(cx);
        }
    }
}
