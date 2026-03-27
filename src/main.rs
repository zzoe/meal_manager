#![windows_subsystem = "windows"]

fn main() {
    // 仅在打包后的可执行文件中设置 MAKEPAD_PACKAGE_DIR
    // 开发运行时让 makepad 使用默认的 crate 资源路径
    if cfg!(not(debug_assertions))
        && std::env::var("MAKEPAD_PACKAGE_DIR").is_err()
        && let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        unsafe {
            std::env::set_var("MAKEPAD_PACKAGE_DIR", exe_dir);
        }
    }

    meal_manager::app::app_main();
}
