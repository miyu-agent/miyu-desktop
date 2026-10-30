//! 网页视图的通知权限（蓝图「怎么走」第 8 条）：页面自己照网页的规矩弹通知，外壳只管网页视图给不给这个权限、点了通知把
//! 窗口拿回来。一个网页视图一份做法，关在这里。

#[cfg(target_os = "linux")]
mod webkitgtk;

/// 放行这个窗口的网页视图的通知。
#[cfg(target_os = "linux")]
pub fn allow(window: &tauri::WebviewWindow) {
    webkitgtk::allow(window);
}

/// WebView2、WKWebView：施工 DT-3 照 CI 出的程序实测，接不住的找网页在 `served.js` 里加。现在什么都不做。
#[cfg(not(target_os = "linux"))]
pub fn allow(_window: &tauri::WebviewWindow) {}
