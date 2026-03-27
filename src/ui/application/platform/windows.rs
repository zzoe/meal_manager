//! Windows 平台窗口处理模块
//!
//! 提供 Windows 平台特定的窗口管理功能：
//! - 基于黄金分割比例的窗口位置计算
//! - 窗口位置和大小设置
//! - 标题栏按钮显示控制

use makepad_widgets::*;

// Windows API imports
use ::windows::Win32::Foundation::RECT;
use ::windows::Win32::Graphics::Gdi::GetMonitorInfoW;
use ::windows::Win32::Graphics::Gdi::MONITOR_DEFAULTTOPRIMARY;
use ::windows::Win32::Graphics::Gdi::MONITORINFO;
use ::windows::Win32::Graphics::Gdi::MonitorFromWindow;
use ::windows::Win32::UI::HiDpi::GetDpiForMonitor;
use ::windows::Win32::UI::HiDpi::MDT_EFFECTIVE_DPI;

/// 黄金分割比例
const GOLDEN_RATIO: f64 = 0.618;

/// 显示器信息
#[derive(Debug)]
struct MonitorInfo {
    /// 工作区左上角 X（物理像素）
    left: f64,
    /// 工作区左上角 Y（物理像素）
    top: f64,
    /// 工作区宽度（物理像素）
    width: f64,
    /// 工作区高度（物理像素）
    height: f64,
    /// DPI 缩放比例（1.0 = 100%）
    dpi_scale: f64,
}

/// 获取主显示器信息
fn get_primary_monitor_info() -> Option<MonitorInfo> {
    unsafe {
        // 获取桌面窗口句柄（始终存在，用于获取主显示器）
        let hwnd = ::windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow();
        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);

        // 获取 DPI
        let mut dpi_x: u32 = 0;
        let mut dpi_y: u32 = 0;
        let dpi_result = GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);
        let dpi_scale = if dpi_result.is_ok() {
            dpi_x as f64 / 96.0
        } else {
            1.0
        };

        // 获取显示器信息
        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT::default(),
            rcWork: RECT::default(),
            dwFlags: 0,
        };

        if GetMonitorInfoW(monitor, &mut monitor_info).as_bool() {
            let width = (monitor_info.rcWork.right - monitor_info.rcWork.left) as f64;
            let height = (monitor_info.rcWork.bottom - monitor_info.rcWork.top) as f64;
            let left = monitor_info.rcWork.left as f64;
            let top = monitor_info.rcWork.top as f64;

            log!(
                "Monitor: physical={}x{} at ({},{}), dpi_scale={}",
                width,
                height,
                left,
                top,
                dpi_scale
            );

            return Some(MonitorInfo {
                left,
                top,
                width,
                height,
                dpi_scale,
            });
        }

        None
    }
}

/// 计算基于黄金分割比例的窗口大小
///
/// 算法：
/// 1. 计算屏幕可用区域的 61.8% 作为约束尺寸
/// 2. 按黄金分割比例计算高度：width * 0.618
/// 3. 如果高度超过约束高度，则按高度反推宽度：height / 0.618
/// 4. 否则使用约束宽度和计算出的高度
///
/// # 参数
/// - `available_width`: 屏幕可用宽度（像素单位）
/// - `available_height`: 屏幕可用高度（像素单位）
///
/// # 返回
/// 返回 `(width, height)`，按黄金分割比例计算的最优窗口大小（与输入单位相同）
fn calc_window_size(available_width: f64, available_height: f64) -> (f64, f64) {
    let constrained_width = available_width * GOLDEN_RATIO;
    let constrained_height = available_height * GOLDEN_RATIO;

    let derived_height = constrained_width * GOLDEN_RATIO;

    let (final_width, final_height) = if derived_height > constrained_height {
        // 按高度约束反推宽度
        let width_from_height = constrained_height / GOLDEN_RATIO;
        (width_from_height, constrained_height)
    } else {
        // 按宽度约束计算高度
        (constrained_width, derived_height)
    };

    log!(
        "Window size calc: constrained={:.0}x{:.0}, derived_height={:.0}, final={:.0}x{:.0}",
        constrained_width,
        constrained_height,
        derived_height,
        final_width,
        final_height
    );

    (final_width, final_height)
}

/// 计算窗口位置和大小
///
/// 窗口水平居中，垂直位置按照黄金分割比例定位
///
/// # 返回
/// 返回 `(position, size)`：
/// - `position` 是物理像素（屏幕坐标），因为 Makepad 直接传递给 Windows API
/// - `size` 是逻辑像素（Makepad 内部使用逻辑像素计算布局）
pub fn calc_window_position() -> (DVec2, DVec2) {
    if let Some(monitor) = get_primary_monitor_info() {
        // 工作区尺寸（物理像素）
        let work_area_width_phys = monitor.width;
        let work_area_height_phys = monitor.height;
        let work_area_left_phys = monitor.left;
        let work_area_top_phys = monitor.top;

        // 直接使用物理像素计算窗口大小（避免不必要的 DPI 转换）
        let (window_width_phys, window_height_phys) =
            calc_window_size(work_area_width_phys, work_area_height_phys);

        // 窗口大小需要转换为逻辑像素供 Makepad 内部使用
        let window_width_log = window_width_phys / monitor.dpi_scale;
        let window_height_log = window_height_phys / monitor.dpi_scale;

        // 水平居中（物理像素）
        let position_x_phys =
            work_area_left_phys + (work_area_width_phys - window_width_phys) / 2.0;

        // 垂直黄金分割（物理像素）- 窗口中心位于黄金分割点
        let golden_division_y_phys = work_area_height_phys * (1.0 - GOLDEN_RATIO);
        let position_y_phys =
            work_area_top_phys + golden_division_y_phys - window_height_phys / 2.0;

        log!(
            "Before clamp: position_x_phys={}, position_y_phys={}, window_width_phys={}, window_height_phys={}",
            position_x_phys,
            position_y_phys,
            window_width_phys,
            window_height_phys
        );

        // 确保窗口不会超出屏幕边界（物理像素）
        let min_position_x = work_area_left_phys + 10.0;
        let min_position_y = work_area_top_phys + 10.0;
        let max_position_x = work_area_left_phys + work_area_width_phys - window_width_phys - 10.0;
        let max_position_y = work_area_top_phys + work_area_height_phys - window_height_phys - 10.0;

        let clamped_x = position_x_phys.clamp(min_position_x, max_position_x);
        let clamped_y = position_y_phys.clamp(min_position_y, max_position_y);

        // position 使用物理像素（给 Windows API）
        let position = dvec2(clamped_x, clamped_y);
        // size 使用逻辑像素（Makepad 内部使用）
        let size = dvec2(window_width_log, window_height_log);

        log!(
            "Window calc: phys pos=({:.1},{:.1}), log size={:.1}x{:.1}, dpi_scale={}",
            clamped_x,
            clamped_y,
            window_width_log,
            window_height_log,
            monitor.dpi_scale
        );

        return (position, size);
    }

    // 默认位置和大小（使用黄金分割比例）
    let fallback_width = 1000.0;
    let fallback_height = fallback_width * GOLDEN_RATIO;
    (dvec2(100.0, 100.0), dvec2(fallback_width, fallback_height))
}

/// 显示 Windows 标题栏按钮
///
/// 在 Windows 平台上显示最小化、最大化、关闭按钮
pub fn show_caption_buttons(ui: &WidgetRef, cx: &mut Cx) {
    ui.widget(&[
        LiveId::from_str("main_window"),
        LiveId::from_str("caption_bar"),
        LiveId::from_str("windows_buttons"),
    ])
    .set_visible(cx, true);
}
