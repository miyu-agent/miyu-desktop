//! Tauri 的编译期准备：读 `tauri.conf.json`、Windows 上把图标编进程序。

fn main() {
    tauri_build::build();
}
