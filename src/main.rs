use meal_manager::Xilem;
use meal_manager::app::{Action, AppState, AppStatus, EditingState, Page};
use meal_manager::dining_analysis::spawn_worker;
use meal_manager::ui::app_logic::app_logic;
use std::sync::{Arc, mpsc};
use xilem::EventLoop;
use xilem::masonry::peniko::Blob;

const PINGFANG_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/fonts/PingFang.ttc"
));

fn main() -> Result<(), winit::error::EventLoopError> {
    // 1. 初始化通信管道
    let (tx_action, rx_action) = crossbeam_channel::unbounded();
    let (tx_event, rx_event) = mpsc::channel();

    // 2. 启动后台 Compio 线程
    spawn_worker(rx_action, tx_event);

    // 3. 初始化默认状态
    let initial_state = AppState {
        current_page: Page::DiningStatistics,
        input_text: "示例：\nEliza: 10\n张三: 20".to_string(),
        current_report: Default::default(),
        employees: vec![],
        editing: EditingState::new_for_add(),
        tx_action: tx_action.clone(),
        status: AppStatus::default(),
        rx_event: rx_event,
    };

    // 初始加载一次数据
    if let Err(e) = tx_action.send(Action::LoadEmployees) {
        eprintln!("初始化加载员工数据失败: {}", e);
        // 继续启动，但初始数据可能为空
    }

    // 4. 启动 Xilem
    let app = Xilem::new(initial_state, move |state: &mut AppState| app_logic(state))
        .with_font(Blob::new(Arc::new(PINGFANG_FONT)));

    app.run_in(EventLoop::with_user_event())
}
