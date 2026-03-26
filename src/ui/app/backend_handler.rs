use crate::employees::{EmployeeAction, EmployeeOp};
use crate::meal_stats::MealAnalysisAction;
use crate::ui::layout::app_shell::AppShellWidgetRefExt;
use crate::ui::pages::employees::employee_page::EmployeePageWidgetRefExt;
use crate::ui::pages::meal_stats::StatsPageWidgetRefExt;
use makepad_widgets::{
    Actions, Cx, LiveId, PageFlipWidgetRefExt, ViewWidgetRefExt, WidgetRef, id, live_id, log,
};

pub fn handle_backend_result(cx: &mut Cx, actions: &Actions, ui: &WidgetRef) {
    let app_shell = ui.widget(&[LiveId::from_str("body")]).as_app_shell();

    for action in actions {
        if let Some(result) = action.downcast_ref::<MealAnalysisAction>() {
            match result {
                MealAnalysisAction::AnalysisComplete { .. } => {
                    let nav_ref = app_shell.view(id!(navigation)).as_page_flip();
                    if let Some(mut nav) = nav_ref.borrow_mut()
                        && let Some(stats_page) = nav.page(cx, live_id!(stats))
                    {
                        stats_page.as_stats_page().update_results(cx, result);
                    }
                }
                MealAnalysisAction::None => {}
            }
        }

        if let Some(result) = action.downcast_ref::<EmployeeAction>() {
            match result {
                EmployeeAction::Loaded(employees) => {
                    let nav_ref = app_shell.view(id!(navigation)).as_page_flip();
                    if let Some(mut nav) = nav_ref.borrow_mut()
                        && let Some(employee_page) = nav.page(cx, live_id!(employees))
                    {
                        employee_page
                            .as_employee_page()
                            .set_employees(cx, employees.to_vec());
                    }
                }
                EmployeeAction::SaveFailed { op, name, error } => {
                    let op_label = match op {
                        EmployeeOp::Add => "新增",
                        EmployeeOp::Update => "更新",
                        EmployeeOp::Delete => "删除",
                    };
                    log!("Employee save failed: op={op_label}, name={name}, error={error}");
                    let msg = format!("{op_label}员工失败: {name}\n{error}");
                    app_shell.show_error(cx, &msg);
                }
                EmployeeAction::None => {}
            }
        }
    }
}
