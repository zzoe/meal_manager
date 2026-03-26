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
                color: (THEME_COLOR_WHITE)
                border_color: (THEME_COLOR_INSET_1)
                border_size: 0.0
                border_radius: 8.0
            }

            <Label> {
                text: "输入点餐数据"
                draw_text: { color: (THEME_COLOR_TEXT), text_style: { font_size: 14.0 } }
            }

            // 输入区域容器：固定边框和底色
            <RoundedView> {
                width: Fill, height: Fill
                flow: Down, margin: { top: 10.0 }
                padding: 10.0
                draw_bg: {
                    color: (THEME_COLOR_WHITE)
                    border_color: (THEME_COLOR_INSET_1)
                    border_size: 1.0
                    border_radius: 8.0
                }

                scroll_view = <ScrollYView> {
                    width: Fill, height: Fill

                    input = <TextInput> {
                        width: Fill, height: Fit
                        padding: 15.0
                        empty_text: "此处粘贴点餐内容，例如:\nzoe: 11\n小明: 01..."
                        is_read_only: false

                        // 背景交给外层 input_container，保持输入区透明以免覆盖边框
                        draw_bg: {
                            color: (THEME_COLOR_WHITE)
                            color_empty: (THEME_COLOR_WHITE)
                            color_focus: (THEME_COLOR_WHITE)
                            color_hover: (THEME_COLOR_WHITE)
                            color_down: (THEME_COLOR_WHITE)
                            border_size: 0.0
                            border_radius: 4.0
                        }

                        draw_text: {
                            text_style: { font_size: 13.0, line_spacing: 1.6 }
                            color: (THEME_COLOR_TEXT)
                            color_empty: (THEME_COLOR_TEXT)
                            color_focus: (THEME_COLOR_TEXT)
                            color_hover: (THEME_COLOR_TEXT)
                            wrap: Word
                        }

                        draw_selection: {
                            color: (THEME_COLOR_SELECTION)
                            color_focus: (THEME_COLOR_SELECTION)
                            color_hover: (THEME_COLOR_SELECTION)
                        }

                        draw_cursor: {
                            color: (THEME_COLOR_TEXT_CURSOR)
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
                    draw_bg: { color: (THEME_COLOR_PRIMARY_LIGHT) }
                    header_label = { text: "🍱 中餐", draw_text: { color: (THEME_COLOR_WARNING) } }
                }
            }

            dinner_card = <ResultCard> {
                header_slot = {
                    draw_bg: { color: (THEME_COLOR_PRIMARY_LIGHTER) }
                    header_label = { text: "🥘 晚餐", draw_text: { color: (THEME_COLOR_PRIMARY) } }
                }
            }

            exception_card = <ResultCard> {
                header_slot = {
                    draw_bg: { color: (THEME_COLOR_SELECTION_HOVER) }
                    header_label = { text: "⚠️ 异常与未报", draw_text: { color: (THEME_COLOR_ERROR) } }
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
        // 点击 ScrollYView 区域时，将焦点设置给 TextInput
        let scroll_view = self.view(id!(scroll_view));
        let text_input = self.text_input(id!(input));

        let scroll_area = scroll_view.area();
        if scroll_area.is_valid(cx)
            && matches!(event.hits(cx, scroll_area), Hit::FingerDown(_))
        {
            text_input.set_key_focus(cx);
        }

        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for StatsPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // 按钮点击或 Enter 键触发分析
        let btn_clicked = self.button(id!(btn_run)).clicked(actions);
        let enter_pressed = self.text_input(id!(input)).returned(actions).is_some();
        if btn_clicked || enter_pressed {
            self.trigger_analysis(cx);
        }

        // 监听 TextInput 的 Changed 事件，自动滚动到光标位置
        let text_input = self.text_input(id!(input));
        if text_input.changed(actions).is_some() {
            self.scroll_to_cursor(cx);
        }
    }
}

impl StatsPage {
    fn trigger_analysis(&mut self, cx: &mut Cx) {
        // 更新按钮文本为加载状态
        self.button(id!(btn_run)).set_text(cx, "正在计算...");
        self.button(id!(btn_run)).set_disabled(cx, true);
        self.button(id!(btn_run)).redraw(cx);

        // 获取输入文本并在后台线程执行分析
        let text = self.text_input(id!(input)).text();
        #[cfg(not(target_arch = "wasm32"))]
        cx.spawn_thread(move || Cx::post_action(analyzer::analyze(text)));
        #[cfg(target_arch = "wasm32")]
        Cx::post_action(analyzer::analyze(text));
    }

    fn scroll_to_cursor(&self, cx: &mut Cx) {
        let scroll_view = self.view(id!(scroll_view));
        let text_input = self.text_input(id!(input));

        // 获取光标位置
        let cursor = text_input.cursor();

        // 获取 ScrollYView 并设置滚动位置
        // ScrollYView 实际上是 View，使用 set_scroll_pos 设置滚动
        let scroll_view_ref = scroll_view.as_view();
        if let Some(mut scroll) = scroll_view_ref.borrow_mut() {
            // 计算目标滚动位置：让光标行位于视口中间偏下
            // 光标 index 对应字符位置，估算行号：按每行约 30 个字符估算
            let row_index = cursor.index / 30; // 粗略估算
            let row_height = 25.0; // approximate row height based on font size 13 + line_spacing 1.6
            let padding_top = 15.0;
            let target_scroll_y = (row_index as f64 * row_height) + padding_top;

            // 设置滚动位置
            scroll.set_scroll_pos(cx, dvec2(0.0, target_scroll_y));
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
