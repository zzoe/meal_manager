use serde::{Deserialize, Serialize};

/// UI 页面枚举
#[derive(Clone, PartialEq, Debug)]
pub enum Page {
    DiningStatistics,
    Settings,
}

impl Default for Page {
    fn default() -> Self {
        Page::DiningStatistics
    }
}

/// 员工数据
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmployeeData {
    pub name: String,
    pub nicknames: String,
}

/// 报表结果
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub lunch_total: u32,
    pub dinner_total: u32,
    pub lunch_details: String,
    pub dinner_details: String,
    pub missing: String,
    pub unknown: String,
}

/// UI 发给后台的指令
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    LoadEmployees,
    SaveEmployee(EmployeeData),
    DeleteEmployee(String),
    Calculate(String),
}

/// Represents the application state.
pub struct AppState {
    pub current_page: Page,
    pub input_text: String,
    pub current_report: Report,
    pub employees: Vec<EmployeeData>,
    pub edit_name: String,
    pub edit_nicks: String,
    pub tx_action: crossbeam_channel::Sender<Action>,
    pub status_msg: String,
}

impl xilem::core::ViewArgument for AppState {
    type Params<'a> = &'a mut AppState;

    fn reborrow_mut<'input, 'a: 'input>(
        params: &'input mut Self::Params<'a>,
    ) -> Self::Params<'input> {
        params
    }
}
