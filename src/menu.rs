//! macOS 的应用菜单（DT-3 定）：Tauri 默认菜单里的「退出」直接退外壳、不经彻底退出，所以自己建。「退出 Miyu」（`⌘Q`）走
//! 彻底退出；「编辑」一栏用系统的，不然 `⌘C`、`⌘V` 在网页视图里不起作用；「窗口」一栏的关闭同关窗口。菜单的事件在
//! `main` 里统一接。

use tauri::AppHandle;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};

use crate::text::{Text, key};

/// 菜单项「退出 Miyu」：和托盘的「退出」同一个编号。
pub const QUIT: &str = crate::tray::QUIT;
/// 菜单项「关闭窗口」。
pub const CLOSE: &str = "close";

/// 建应用菜单。
///
/// # Errors
/// 建不出菜单。
pub fn create(app: &AppHandle, text: &Text) -> tauri::Result<Menu<tauri::Wry>> {
    let quit = MenuItem::with_id(
        app,
        QUIT,
        text.get(key::MENU_QUIT),
        true,
        Some("CmdOrCtrl+Q"),
    )?;
    let close = MenuItem::with_id(
        app,
        CLOSE,
        text.get(key::MENU_CLOSE),
        true,
        Some("CmdOrCtrl+W"),
    )?;
    let application = Submenu::with_items(
        app,
        "Miyu",
        true,
        &[
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::show_all(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;
    let edit = Submenu::with_items(
        app,
        text.get(key::MENU_EDIT),
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;
    let window = Submenu::with_items(
        app,
        text.get(key::MENU_WINDOW),
        true,
        &[&PredefinedMenuItem::minimize(app, None)?, &close],
    )?;
    Menu::with_items(app, &[&application, &edit, &window])
}
