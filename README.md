## Miyu 桌面端

开箱即用的 Miyu：装上就能用，不用开终端。窗口里是 Miyu 的网页，关掉窗口她在托盘里照旧干活，做完了、要你确认会弹系统通知；托盘右键「退出」连核心一起关掉。

- 是网页的薄封装：窗口开的是网页软件 [miyu-agent/miyu-web](https://github.com/miyu-agent/miyu-web) 给的页面，核心在 [miyu-agent/miyu](https://github.com/miyu-agent/miyu)。
- 外壳用 [Tauri](https://tauri.app)：系统自带的网页视图加一个 Rust 的程序。
- 支持 Linux、macOS（M 系列）、Windows。
- 终端界面不在这里，另装 [miyu-agent/miyu-tui](https://github.com/miyu-agent/miyu-tui)。

施工排期见 [`docs/construction/README.md`](docs/construction/README.md)。

### 许可证

AGPL-3.0-or-later，见 [`LICENSE`](LICENSE)。编进外壳的第三方包的版权和许可证原文见 [`THIRD-PARTY-LICENSES.txt`](THIRD-PARTY-LICENSES.txt)（`cargo xtask notices` 生成）；安装包的资源目录下 `licenses/` 里放外壳、核心、网页三份。
