//! 托盘（蓝图「怎么走」第 11 条）：单色图标，菜单「显示 Miyu」「退出」。左键点图标同「显示 Miyu」（Linux 上托盘库收不到
//! 图标的点击，只有菜单）。菜单的事件在 `main` 里统一接。

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, image::Image};

use crate::text::{Text, key};
use crate::window;

/// 托盘图标（施工 DT-5，`icons/source/build.sh` 出的）：macOS 是模板图，系统照菜单栏着色；别的带浅色底板，深色、浅色的任务栏上
/// 都看得见。
#[cfg(target_os = "macos")]
const TRAY_ICON: &[u8] = include_bytes!("../icons/tray-template.png");
#[cfg(not(target_os = "macos"))]
const TRAY_ICON: &[u8] = include_bytes!("../icons/tray.png");

/// 托盘图标的编号。
pub const ID: &str = "main";
/// 菜单项「显示 Miyu」。
pub const SHOW: &str = "show";
/// 菜单项「退出」（彻底退出）。
pub const QUIT: &str = "quit";

/// 建托盘图标。
///
/// # Errors
/// 建不出（Linux 上没有 `libayatana-appindicator` 这些）。
pub fn create(app: &AppHandle, text: &Text) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, SHOW, text.get(key::TRAY_SHOW), true, None::<&str>)?;
    let quit = MenuItem::with_id(app, QUIT, text.get(key::TRAY_QUIT), true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;
    let icon = Image::from_bytes(TRAY_ICON)?;
    TrayIconBuilder::with_id(ID)
        .icon(icon)
        .icon_as_template(true)
        .tooltip("Miyu")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                window::bring_back(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}
