use crate::services::{MealAnalysisResult, analyze_meal, load_config, save_config};
use crate::ui::layout::app_shell::{AppAction, AppShellWidgetRefExt};
use crate::ui::pages::stats_page::StatsPageWidgetRefExt;
use crate::ui::pages::config_page::ConfigPageWidgetRefExt;
use compio::dispatcher::Dispatcher;
use makepad_widgets::{Cx, WidgetRef, LiveId, ViewWidgetRefExt, id, live_id, PageFlipWidgetRefExt};

pub struct AppHandler;

impl AppHandler {
    pub fn handle_ui_action(
        cx: &mut Cx,
        action: &AppAction,
        ui: &WidgetRef,
        dispatcher: &Dispatcher,
    ) {
        let app_shell = ui.widget(&[
            LiveId::from_str("main_window"), 
            LiveId::from_str("body")
        ]).as_app_shell();

        match action {
            AppAction::NavigateToStats => {
                app_shell.show_page(cx, "stats");
            }
            AppAction::NavigateToConfig => {
                app_shell.show_page(cx, "config");
                let _ = dispatcher.dispatch_blocking(move || {
                    load_config();
                });
            }
            AppAction::SubmitAnalysis(text) => {
                let text = text.clone();
                if let Some(mut page_flip) = app_shell.view(id!(navigation)).as_page_flip().borrow_mut() {
                    if let Some(stats_page) = page_flip.page(cx, live_id!(stats)) {
                        stats_page.as_stats_page().set_loading_status(cx, "正在计算...");
                    }
                }
                let _ = dispatcher.dispatch_blocking(move || {
                    analyze_meal(text);
                });
            }
            AppAction::SaveConfig(text) => {
                let text = text.clone();
                let _ = dispatcher.dispatch_blocking(move || {
                    save_config(text);
                });
            }
            _ => (),
        }
    }

    pub fn handle_backend_result(cx: &mut Cx, result: &MealAnalysisResult, ui: &WidgetRef) {
        let app_shell = ui.widget(&[LiveId::from_str("body")]).as_app_shell();

        match result {
            MealAnalysisResult::Success {
                lunch_summary,
                lunch_details,
                dinner_summary,
                dinner_details,
                exception_summary,
                exception_details,
            } => {
                if let Some(mut page_flip) = app_shell.view(id!(navigation)).as_page_flip().borrow_mut() {
                    if let Some(stats_page) = page_flip.page(cx, live_id!(stats)) {
                        stats_page.as_stats_page().reset_loading_status(cx);
                        stats_page.as_stats_page().update_results(
                            cx,
                            lunch_summary,
                            lunch_details,
                            dinner_summary,
                            dinner_details,
                            exception_summary,
                            exception_details,
                        );
                    }
                }
            }
            MealAnalysisResult::ConfigLoaded(text) => {
                if let Some(mut page_flip) = app_shell.view(id!(navigation)).as_page_flip().borrow_mut() {
                    if let Some(config_page) = page_flip.page(cx, live_id!(config)) {
                        config_page.as_config_page().set_config_text(cx, text);
                    }
                }
            }
            MealAnalysisResult::ConfigSaved => {
                if let Some(mut page_flip) = app_shell.view(id!(navigation)).as_page_flip().borrow_mut() {
                    if let Some(config_page) = page_flip.page(cx, live_id!(config)) {
                        config_page.redraw(cx);
                    }
                }
            }
            MealAnalysisResult::None => {}
        }
    }
}
