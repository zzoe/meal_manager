use meal_manager::app::{AppState, Action, AppEvent, Page};
use meal_manager::dining_analysis::spawn_worker;
use meal_manager::ui::app_logic::app_logic;
use meal_manager::Xilem;

fn main() {
    // 1. 初始化通信管道
    let (tx_action, rx_action) = crossbeam_channel::unbounded();
    let (tx_event, rx_event) = std::sync::mpsc::channel();

    // 2. 启动后台 Compio 线程
    spawn_worker(rx_action, tx_event);

    // 3. 初始化默认状态
    let initial_state = AppState {
        current_page: Page::DiningStatistics,
        input_text: "示例：\nEliza: 10\n张三: 20".to_string(),
        current_report: Default::default(),
        employees: vec![],
        edit_name: "".into(),
        edit_nicks: "".into(),
        tx_action: tx_action.clone(),
        status_msg: "就绪".into(),
    };

    // 初始加载一次数据
    tx_action.send(Action::LoadEmployees).unwrap();

    // 4. 启动 Xilem
    let app = Xilem::new(initial_state, |state: &mut AppState| {
        // --- 核心：事件轮询 (在每次 UI 更新时检查后台消息) ---
        while let Ok(event) = rx_event.try_recv() {
            match event {
                AppEvent::EmployeesLoaded(list) => {
                    state.employees = list;
                }
                AppEvent::ReportReady(report) => {
                    state.current_report = report;
                    state.status_msg = "计算完成".into();
                }
                AppEvent::StatusMessage(msg) => {
                    state.status_msg = msg;
                }
            }
        }
        app_logic(state)
    });

    app.run_windowed(Xilem::with_user_event(), "报餐助手 Pro".into()).unwrap();
}
