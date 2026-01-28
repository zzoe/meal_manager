use crate::ui::app::AppAction;
use crate::ui::layout::app_shell::AppShellWidgetRefExt;
use makepad_widgets::*;

pub fn handle_ui_action(cx: &mut Cx, action: &AppAction, ui: &WidgetRef) {
    let app_shell = ui
        .widget(&[LiveId::from_str("main_window"), LiveId::from_str("body")])
        .as_app_shell();

    match action {
        AppAction::NavigateToStats => {
            app_shell.show_page(cx, "stats");
        }
        AppAction::NavigateToConfig => {
            app_shell.show_page(cx, "config");
        }
        _ => (),
    }
}
