//! WebKitGTK：默认直接拒绝通知的权限请求（DT-1）。接 `permission-request`：通知的放行，别的（定位、摄像头、麦克风……）
//! 一律拒。接 `show-notification`：通知照默认的样子弹，点了把窗口拿回来。

use glib::prelude::Cast;
use tauri::Manager;
use webkit2gtk::{
    NotificationExt, NotificationPermissionRequest, PermissionRequestExt, WebViewExt,
};

use crate::{log, window};

pub fn allow(window: &tauri::WebviewWindow) {
    let app = window.app_handle().clone();
    let hooked = window.with_webview(move |webview| {
        let view = webview.inner();
        view.connect_permission_request(|_, request| {
            if request
                .downcast_ref::<NotificationPermissionRequest>()
                .is_some()
            {
                request.allow();
            } else {
                log::line("a permission other than notifications was refused");
                request.deny();
            }
            true
        });
        view.connect_show_notification(move |_, notification| {
            let app = app.clone();
            notification.connect_clicked(move |_| window::bring_back(&app));
            false
        });
    });
    if let Err(error) = hooked {
        log::line(&format!("hooking notifications: {error}"));
    }
}
