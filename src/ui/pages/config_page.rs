use crate::services::save_config;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::components::common::*;

    // 配置页面
    pub ConfigPage = {{ConfigPage}} {
        width: Fill, height: Fill
        flow: Down, spacing: 20.0, padding: 40.0

        <View> {
            width: Fill, height: Fit
            flow: Down, spacing: 5.0
            <Label> {
                text: "员工配置"
                draw_text: { color: (COLOR_TEXT_PRIMARY), text_style: { font_size: 18.0 } }
            }
            <Label> {
                text: "姓名: 昵称1, 昵称2"
                draw_text: { color: (COLOR_TEXT_SECONDARY), text_style: { font_size: 12.0 } }
            }
        }

        <RoundedView> {
            width: Fill, height: Fill
            draw_bg: {
                color: (COLOR_BG_CARD)
                border_color: (COLOR_BORDER)
                border_size: 1.0
                border_radius: 8.0
            }
            padding: 10.0

            <ScrollYView> {
                width: Fill, height: Fill
                config_input = <CleanTextInput> {
                    text: "加载中..."
                }
            }
        }

        <View> {
            width: Fill, height: Fit, align: {x: 1.0}
            btn_save_config = <RedButton> { width: 150.0, text: "保存配置" }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ConfigPage {
    #[deref]
    view: View,
}

impl Widget for ConfigPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ConfigPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(id!(btn_save_config)).clicked(actions) {
            self.button(id!(btn_save_config)).set_disabled(cx, true);
            // 获取配置文本并在后台线程保存
            let text = self.text_input(id!(config_input)).text();
            cx.spawn_thread(move || save_config(text));
        }
    }
}

impl ConfigPageRef {
    pub fn set_config_text(&self, cx: &mut Cx, text: &str) {
        if let Some(inner) = self.borrow() {
            let input = inner.text_input(id!(config_input));
            input.set_text(cx, text);
            input.redraw(cx);
        }
    }

    pub fn reset_btn_save_config(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow() {
            let btn = inner.button(id!(btn_save_config));
            btn.set_disabled(cx, false);
            btn.redraw(cx);
        }
    }
}
