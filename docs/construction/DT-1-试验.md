## DT-1：试验，Tauri 窗口直接开网页软件的页面

状态：已完成（2026-10-30，Linux）。代码不进仓库，结果记在这里；macOS、Windows 等 DT-3 的三平台 CI 出了试验包再看。

### 目的

先在 Linux 上确认「Tauri 2 窗口直接打开 `miyu-web` 给的页面」这条路走得通：WebKitGTK 的渲染和流畅、窗口藏起来以后占多少内存、还收不收得到推送，外链、新窗口、下载、通知、剪贴板接不接得住。OpenCode 的桌面端走过同一条路，后来因为 WebKit 慢、样式有出入换成了 Electron，所以要先量。

### 怎么做的

Arch，niri（Wayland），webkit2gtk-4.1 2.54.1；tauri 2.12.5、wry 0.57.1、tauri-plugin-opener 2.7.2；核心 7098c9047、网页 96d56fc6。新的空数据根，没有模型的 key，页面停在第一次的引导。外壳是一个最小的 Tauri 程序：起来时调 `miyu-web open --print --format json --ticket` 拿网址，导航拦截、新窗口交系统浏览器、`on_download`、关窗口藏起来、托盘。

### 结果

| 项 | 结论 | 依据 |
|---|---|---|
| 拿网址 | 行 | 一行 JSON 带 `#ticket=`，3 到 17 毫秒 |
| 渲染 | 行 | 引导页字体、图标、布局和 Chromium 里同一网址看不出差别，深浅色切换正常 |
| 控制台 | 行 | 页面自己零报错，CSP 没有违规 |
| 流畅 | 行 | 没有白屏、DMABUF 报错、闪烁；静止页 131 到 139 帧每秒（165Hz 屏），每帧中位 6 毫秒 |
| 藏起来收推送 | 行 | 藏着时改 `ui.language`，页面约 1 毫秒收到 `config.changed`、照自己的规矩重载、用存着的设备令牌重连；再显示已是英文；WebSocket 一直没断 |
| 藏着的开销 | 行 | `document.hidden` 为真，动画 0 帧，CPU 10 秒 1 个时钟滴答 |
| 外链、新窗口 | 行，要改一处 | 同源放行，别的交 `xdg-open`；opener 插件默认接管 `target=_blank` 走 IPC，远程来源的页面没权限、点了没反应，要 `open_js_links_on_click(false)`，交给 `on_new_window` |
| 下载 | 行 | `on_download` 拿得到建议的文件名，`<a download>`、blob URL、不写名字的三种都存下了；靠 `Content-Disposition` 给名字的 `/media` 没试 |
| 通知 | 部分行 | WebKitGTK 默认 `requestPermission()` 直接 `denied`，Tauri 没有放行的接口；外壳接 webkit2gtk 的 `permission-request` 放行通知以后真弹出来了 |
| 剪贴板 | 行 | `navigator.clipboard.writeText` 成 |
| 关窗口 | 行 | 拦下关闭、藏起来，进程留着；再显示页面状态还在、不重载 |
| 托盘 | 部分行 | 右键菜单「显示」「退出」管用；左键点图标在 Linux 上收不到事件（tray-icon 在 Linux 上不支持图标的点击），只能靠菜单 |

内存（外壳加 WebKit 的子进程，Pss 合计；外壳是 debug 版，release 会小很多）：

| 时刻 | 合计 | 外壳 | 网络进程 | 网页进程 |
|---|---|---|---|---|
| 显示时 | 715 MiB | 164 MiB | 40 MiB | 511 MiB |
| 藏起来 1 分钟 | 458 MiB | 164 MiB | 42 MiB | 253 MiB |
| 藏起来 5 分钟 | 404 MiB | 147 MiB | 36 MiB | 222 MiB |

同一页面在 Chromium 里 588 MiB。显示时引导页的动画一直在跑：外壳约 10% 核、网页进程约 17%。

### 定下的（技术细节，照推荐）

1. Tauri 这条路在 Linux 上走得下去，DT-3 照它写外壳。
2. 外壳关掉 Tauri 对远程来源页面的 IPC 接管：opener 插件不接管链接点击，外链、新窗口一律经 `on_navigation`、`on_new_window` 交系统浏览器。页面里不用 `window.__TAURI__`。
3. 通知权限由外壳照平台放行：Linux 接 webkit2gtk 的权限请求；macOS 的网页视图没有 `Notification`，到时找网页在 `served.js` 那一份宿主里加。
4. Linux 上托盘只靠右键菜单；Windows、macOS 左键照常显示窗口。
5. 网页视图的数据目录（`localStorage` 里的设备令牌）用 Tauri 的 `data_directory` 指到外壳自己的配置目录，不放进 Miyu 的数据根。

### 没测、留给后面的

- release 版外壳的内存；WebKit 的内存压力设置能省多少（DT-3 量）。
- 别的发行版上更老的 WebKitGTK、X11、NVIDIA 下的 DMABUF、默认不显示托盘的 GNOME。
- macOS、Windows：等 DT-3 的 CI 出试验包。
