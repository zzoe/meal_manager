use crate::backend::meal_service::{
    MealAnalysisResult, analyze_meal_data, load_config_task, save_config_task,
};
use crate::components::meal_view::{MealUiAction, MealViewWidgetRefExt};
use compio::dispatcher::Dispatcher;
use makepad_widgets::*;

pub struct MealHandler;

impl MealHandler {
    pub fn handle_ui_action(
        cx: &mut Cx,
        action: &MealUiAction,
        ui: &WidgetRef,
        dispatcher: &Dispatcher,
    ) {
        let meal_view = ui.view(id!(body)).as_meal_view();

        match action {
            MealUiAction::NavToStats => meal_view.show_page(cx, "stats"),
            MealUiAction::NavToConfig => {
                meal_view.show_page(cx, "config");
                let _ = dispatcher.dispatch_blocking(move || load_config_task());
            }
            MealUiAction::SubmitText(text) => {
                let text = text.clone();
                meal_view.set_loading_status(cx, "计算中...");
                let _ = dispatcher.dispatch_blocking(move || analyze_meal_data(text));
            }
            MealUiAction::SaveConfig(text) => {
                let text = text.clone();
                let _ = dispatcher.dispatch_blocking(move || save_config_task(text));
            }
            _ => (),
        }
    }

    pub fn handle_backend_result(cx: &mut Cx, result: &MealAnalysisResult, ui: &WidgetRef) {
        let meal_view = ui.view(id!(body)).as_meal_view();

        match result {
            MealAnalysisResult::Success {
                lunch_summary, lunch_details,
                dinner_summary, dinner_details,
                exception_summary, exception_details,
                .. // 忽略 summary
            } => {
                meal_view.reset_loading_status(cx);
                meal_view.update_results(
                    cx,
                    lunch_summary, lunch_details,
                    dinner_summary, dinner_details,
                    exception_summary, exception_details
                );
            }
            MealAnalysisResult::ConfigLoaded(text) => {
                meal_view.set_config_text(cx, text);
            }
            MealAnalysisResult::ConfigSaved => {
                meal_view.redraw(cx);
            }
            _ => (),
        }
    }
}
