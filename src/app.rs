use crate::backend::meal_service::MealAnalysisResult;
use crate::components::meal_view::MealUiAction;
use crate::handlers::meal_handler::MealHandler;
use compio::dispatcher::Dispatcher;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::components::meal_view::MealView;

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(800, 600)},
            pass: {clear_color: #F2E6D8}

            body = <MealView> {}
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust(Dispatcher::builder().worker_threads(std::num::NonZeroUsize::new(2).unwrap()).build().unwrap())]
    dispatcher: Dispatcher,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::components::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let Some(widget_action) = action.as_widget_action() {
                let act = widget_action.cast::<MealUiAction>();
                match act {
                    MealUiAction::None => (),
                    _ => MealHandler::handle_ui_action(cx, &act, &self.ui, &self.dispatcher),
                }
            }

            let result = makepad_widgets::ActionCast::<MealAnalysisResult>::cast(action);
            match result {
                MealAnalysisResult::None => (),
                _ => MealHandler::handle_backend_result(cx, &result, &self.ui),
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
