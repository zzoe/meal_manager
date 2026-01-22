use crate::services::BackendResult;
use crate::ui::layout::app_shell::AppShellWidgetRefExt;
use crate::ui::pages::config_page::ConfigPageWidgetRefExt;
use crate::ui::pages::stats_page::StatsPageWidgetRefExt;
use fastant::Instant;
use makepad_widgets::{Cx, LiveId, PageFlipWidgetRefExt, ViewWidgetRefExt, WidgetRef, id, live_id};

pub fn handle_backend_result(cx: &mut Cx, result: &BackendResult, ui: &WidgetRef) {
    let app_shell = ui.widget(&[LiveId::from_str("body")]).as_app_shell();

    match result {
        BackendResult::AnalysisComplete {
            lunch_summary,
            lunch_details,
            dinner_summary,
            dinner_details,
            exception_summary,
            exception_details,
        } => {
            if let Some(mut page_flip) = app_shell.view(id!(navigation)).as_page_flip().borrow_mut()
            {
                if let Some(stats_page) = page_flip.page(cx, live_id!(stats)) {
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
        BackendResult::ConfigLoaded(text) => {
            let start = Instant::now();
            if let Some(mut page_flip) = app_shell.view(id!(navigation)).as_page_flip().borrow_mut()
            {
                if let Some(config_page) = page_flip.page(cx, live_id!(config)) {
                    config_page.as_config_page().set_config_text(cx, text);
                }
            }
            println!("ConfigLoaded处理耗时: {:?}", start.elapsed());
        }
        BackendResult::ConfigSaved => {
            if let Some(mut page_flip) = app_shell.view(id!(navigation)).as_page_flip().borrow_mut()
            {
                if let Some(config_page) = page_flip.page(cx, live_id!(config)) {
                    config_page.as_config_page().reset_btn_save_config(cx);
                }
            }
        }
        BackendResult::None => {}
    }
}
