use crate::services::{MealAnalysisResult, load_config};
use crate::ui::layout::app_shell::{AppAction, AppShellWidgetRefExt};
use crate::ui::handlers::AppHandler;
use compio::dispatcher::Dispatcher;
use makepad_widgets::*;


live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::ui::layout::app_shell::AppShell;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {inner_size: vec2(1000, 600)},
                pass: {clear_color: #F2E6D8}

                caption_bar = {
                    visible: true,
                    margin: {left: -200},
                    caption_label = {
                        label = {
                            text: "Meal Manager"
                            draw_text: { color: #000 }
                        }
                    }
                },

                body = <AppShell> {}
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust(Dispatcher::new().unwrap())]
    dispatcher: Dispatcher,
    #[rust(Vec::new())]
    precompile_queue: Vec<String>,
    #[rust(None)]
    current_precompile_page: Option<String>,
    #[rust(0)]
    precompile_frames_remaining: i32,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::ui::register_live_design(cx);
    }
}

impl App {
    fn start_next_precompile(&mut self, cx: &mut Cx) {
        if let Some(page) = self.precompile_queue.pop() {
            self.current_precompile_page = Some(page.clone());
            // 切换到该页面以触发GPU编译
            let app_shell = self.ui.widget(&[
                LiveId::from_str("main_window"), 
                LiveId::from_str("body")
            ]).as_app_shell();
            app_shell.show_page(cx, &page);
            println!("开始预编译页面: {}", page);
            // 保持5帧以提供足够的GPU编译时间（经验值）
            self.precompile_frames_remaining = 5;
            cx.new_next_frame();
        } else {
            // 所有页面预编译完成，确保回到默认页（stats）
            let app_shell = self.ui.widget(&[
                LiveId::from_str("main_window"), 
                LiveId::from_str("body")
            ]).as_app_shell();
            app_shell.show_page(cx, "stats");
            self.current_precompile_page = None;
            println!("所有页面预编译完成");
        }
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // 应用启动时异步加载配置
        let _ = self.dispatcher.dispatch_blocking(move || {
            load_config();
            println!("config loaded")
        });
        
        // 初始化预编译队列（包含所有页面）
        // 注意：stats页是默认页，但为了完整性也加入队列
        self.precompile_queue = vec!["stats".to_string(),"config".to_string()];
        self.start_next_precompile(cx);
    }
    
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let Some(widget_action) = action.as_widget_action() {
                let act = widget_action.cast::<AppAction>();
                match act {
                    AppAction::None => (),
                    _ => AppHandler::handle_ui_action(cx, &act, &self.ui, &self.dispatcher),
                }
            }

            let result = makepad_widgets::ActionCast::<MealAnalysisResult>::cast(action);
            match result {
                MealAnalysisResult::None => (),
                _ => AppHandler::handle_backend_result(cx, &result, &self.ui),
            }
        }
    }
    
    fn handle_next_frame(&mut self, cx: &mut Cx, _e: &NextFrameEvent) {
        if self.precompile_frames_remaining > 0 {
            self.precompile_frames_remaining -= 1;
            if self.precompile_frames_remaining == 0 {
                // 当前页面编译完成，开始下一个
                println!("页面预编译完成: {:?}", self.current_precompile_page);
                self.start_next_precompile(cx);
            } else {
                // 继续等待下一帧
                cx.new_next_frame();
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
