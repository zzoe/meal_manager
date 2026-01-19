use makepad_widgets::*;
use crate::ui::layout::app_shell::AppAction;

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
                border_size: 1.0
                border_radius: 8.0
            }

            <Label> {
                text: "数据录入"
                draw_text: { color: (COLOR_TEXT_PRIMARY), text_style: { font_size: 14.0 } }
            }

            <Label> {
                text: "Shift+Enter 换行"
                draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 11.0 } }
            }

            // 输入框容器
            <RoundedView> {
                width: Fill, height: Fill
                draw_bg: {
                    color: #F9FAFB
                    border_color: (COLOR_BORDER)
                    border_size: 1.0
                    border_radius: 4.0
                }
                padding: 8.0

                <ScrollYView> {
                    width: Fill, height: Fill
                    input_box = <CleanTextInput> {
                        text: ""
                    }
                }
            }

            btn_run = <RedButton> { text: "开始分析" }
        }

        // 列 2：结果仪表盘
        <View> {
            width: Fill, height: Fill
            flow: Down, spacing: 10.0

            // 1. 🍱 中餐 (x份)
            <View> {
                width: Fill, height: Fill,
                lunch_card = <ResultCard> {
                    header_slot = {
                        draw_bg: { color: (COLOR_LUNCH_HEADER) }
                        header_label = { text: "🍱 中餐", draw_text: { color: (COLOR_LUNCH_TEXT) } }
                    }
                }
            }

            // 2. 🥘 晚餐 (y份)
            <View> {
                width: Fill, height: Fill,
                dinner_card = <ResultCard> {
                    header_slot = {
                        draw_bg: { color: (COLOR_DINNER_HEADER) }
                        header_label = { text: "🥘 晚餐", draw_text: { color: (COLOR_DINNER_TEXT) } }
                    }
                }
            }

            // 3. ⚠️ 异常监控 (z条)
            <View> {
                width: Fill, height: Fill,
                exception_card = <ResultCard> {
                    header_slot = {
                        draw_bg: { color: (COLOR_ERROR_HEADER) }
                        header_label = { text: "⚠️ 异常监控", draw_text: { color: (COLOR_ERROR_TEXT) } }
                    }
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
        let uid = self.widget_uid();

        if self.button(id!(btn_run)).clicked(actions) {
            let text = self.text_input(id!(input_box)).text();
            cx.widget_action(
                uid,
                &HeapLiveIdPath::default(),
                AppAction::SubmitAnalysis(text),
            );
        }
    }
}

impl StatsPageRef {
    pub fn update_results(
        &self,
        cx: &mut Cx,
        lunch_title: &str,
        lunch_details: &str,
        dinner_title: &str,
        dinner_details: &str,
        exception_title: &str,
        exception_details: &str,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .label(id!(lunch_card.header_slot.header_label))
                .set_text(cx, lunch_title);
            inner
                .label(id!(dinner_card.header_slot.header_label))
                .set_text(cx, dinner_title);
            inner
                .label(id!(exception_card.header_slot.header_label))
                .set_text(cx, exception_title);

            inner
                .label(id!(lunch_card.content_view.content))
                .set_text(cx, lunch_details);
            inner
                .label(id!(dinner_card.content_view.content))
                .set_text(cx, dinner_details);
            inner
                .label(id!(exception_card.content_view.content))
                .set_text(cx, exception_details);

            inner.redraw(cx);
        }
    }

    pub fn reset_loading_status(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.button(id!(btn_run)).set_text(cx, "开始分析");
            inner.redraw(cx);
        }
    }

    pub fn set_loading_status(&self, cx: &mut Cx, msg: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.button(id!(btn_run)).set_text(cx, msg);
            inner.redraw(cx);
        }
    }
}
