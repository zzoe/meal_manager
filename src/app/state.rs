use crate::app::events::AppEvent;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use xilem::core::ViewArgument;

/// UI 页面枚举
#[derive(Clone, Default, PartialEq, Debug)]
pub enum Page {
    #[default]
    DiningStatistics,
    Settings,
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
    UpdateEmployee {
        old_name: String,
        new_data: EmployeeData,
    },
    Calculate(String),
}

/// 编辑状态，封装所有编辑相关的字段
#[derive(Clone, Debug, Default)]
pub struct EditingState {
    /// 正在编辑的员工原始名称（如果为编辑模式）
    pub original_name: Option<String>,
    /// 编辑中的员工名称
    pub name: String,
    /// 编辑中的昵称列表
    pub nicknames: String,
    /// 是否为添加新员工模式（false则为编辑现有员工模式）
    pub is_adding_new: bool,
}

impl EditingState {
    /// 创建一个新的添加员工状态
    pub fn new_for_add() -> Self {
        Self {
            original_name: None,
            name: String::new(),
            nicknames: String::new(),
            is_adding_new: true,
        }
    }

    /// 创建一个编辑现有员工状态
    pub fn new_for_edit(original_name: String, current_name: String, nicknames: String) -> Self {
        Self {
            original_name: Some(original_name),
            name: current_name,
            nicknames,
            is_adding_new: false,
        }
    }

    /// 重置编辑状态
    pub fn reset(&mut self) {
        self.original_name = None;
        self.name.clear();
        self.nicknames.clear();
        self.is_adding_new = true;
    }

    /// 是否正在编辑（包括添加或修改）
    pub fn is_editing(&self) -> bool {
        !self.name.is_empty() || !self.nicknames.is_empty()
    }
}

/// 应用状态，封装应用级状态信息
#[derive(Clone, Debug)]
pub struct AppStatus {
    /// 状态消息
    pub message: String,
    /// 窗口ID
    pub window_id: xilem::WindowId,
    /// 应用是否正在运行
    pub is_running: bool,
}

impl Default for AppStatus {
    fn default() -> Self {
        Self {
            message: "就绪".into(),
            window_id: xilem::WindowId::next(),
            is_running: true,
        }
    }
}

/// 主应用状态
pub struct AppState {
    /// 当前页面
    pub current_page: Page,
    /// 用户输入文本
    pub input_text: String,
    /// 当前分析报告
    pub current_report: Report,
    /// 员工列表
    pub employees: Vec<EmployeeData>,
    /// 编辑状态
    pub editing: EditingState,
    /// 发送操作到后台的通道
    pub tx_action: crossbeam_channel::Sender<Action>,
    /// 应用状态
    pub status: AppStatus,
    /// 接收事件的通道
    pub rx_event: mpsc::Receiver<AppEvent>,
}

impl AppState {
    pub fn tick(&mut self) {
        while let Ok(event) = self.rx_event.try_recv() {
            match event {
                AppEvent::EmployeesLoaded(list) => {
                    self.employees = list;
                }
                AppEvent::ReportReady(report) => {
                    self.current_report = report;
                    self.status.message = "计算完成".into();
                }
                AppEvent::StatusMessage(msg) => {
                    self.status.message = msg;
                }
            }
        }
    }
}

impl ViewArgument for AppState {
    type Params<'a> = &'a mut AppState;

    fn reborrow_mut<'input, 'a: 'input>(
        params: &'input mut Self::Params<'a>,
    ) -> Self::Params<'input> {
        params
    }
}

impl xilem::AppState for AppState {
    fn keep_running(&self) -> bool {
        self.status.is_running
    }
}
