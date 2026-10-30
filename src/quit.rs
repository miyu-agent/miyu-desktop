//! 彻底退出（蓝图「怎么走」第 12 条）：托盘「退出」、macOS 的「退出 Miyu」（`⌘Q`）。先藏窗口、拿掉托盘图标，调
//! `miyu core stop` 叫核心停，再退出外壳；核心没停成照样退。网页软件不管，它照自己的规矩空闲了退。

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{AppHandle, Manager};

use crate::program::{self, RunError};
use crate::{log, tray, window};

static QUITTING: AtomicBool = AtomicBool::new(false);

/// 连核心一起退。点了两次的只算一次。
pub fn everything(app: &AppHandle, miyu: Option<PathBuf>) {
    if QUITTING.swap(true, Ordering::SeqCst) {
        return;
    }
    log::line("quitting with the core");
    if let Some(window) = app.get_webview_window(window::LABEL)
        && let Err(error) = window.hide()
    {
        log::line(&format!("hiding the window: {error}"));
    }
    if app.remove_tray_by_id(tray::ID).is_none() {
        log::line("no tray icon to remove");
    }
    let app = app.clone();
    std::thread::spawn(move || {
        match miyu {
            Some(miyu) => stop_core(&miyu),
            None => log::line("main program not found, the core is left alone"),
        }
        app.exit(0);
    });
}

fn stop_core(miyu: &std::path::Path) {
    match program::run(miyu, program::STOP, program::STOP_LIMIT) {
        Ok(ran) if ran.code == Some(0) => log::line("core stopped"),
        Ok(ran) => log::line(&format!(
            "core stop exited {:?}: {}",
            ran.code,
            program::tail(&ran.stderr)
        )),
        Err(RunError::Timeout) => log::line("core stop timed out"),
        Err(RunError::Spawn(error) | RunError::Wait(error)) => {
            log::line(&format!("core stop: {error}"))
        }
    }
}
