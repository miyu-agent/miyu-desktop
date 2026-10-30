//! Tauri 的编译期准备：读 `tauri.conf.json`、Windows 上把图标编进程序；外壳自己的命令只有一条 `notify`（施工 DT-8），在这里
//! 登记，生成 `allow-notify` 这个权限，`capabilities/page-notify.json` 只把它给页面。

fn main() {
    let attributes = tauri_build::Attributes::new()
        .app_manifest(tauri_build::AppManifest::new().commands(&["notify"]));
    if let Err(error) = tauri_build::try_build(attributes) {
        eprintln!("tauri-build: {error:#}");
        std::process::exit(1);
    }
}
