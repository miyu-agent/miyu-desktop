## 桌面端的外壳

状态：草案（施工 DT-2，2026-10-30）。项目主人 2026-10-29 定的七条见 `docs/construction/README.md` 第一节；核心仓库的设计 `21-网页.md` X9 记的是为什么这样做，这一页写外壳怎么做。

### 它是什么

一个 Tauri 2 程序 `miyu-desktop`：一个窗口、一个托盘图标。窗口里开的是网页软件 `miyu-web` 在本机给的页面，和浏览器里打开的是同一份；页面照网页仓库浏览器的那一份宿主（`src/host/served.js`）跑，连核心、媒体地址、软件后台页都由网页软件给。外壳不连核心、不读本机令牌、不依赖核心和网页的 crate，只调装在旁边的主程序 `miyu`：

| 什么时候 | 调什么 | 拿到什么 |
|---|---|---|
| 开窗口 | `miyu web --print --format json --ticket` | 一行 `{"url": …, "first": …}`；网址是 `http://127.0.0.1:<口子>/#ticket=<票>`，票 60 秒、一次性（核心 LT-1，网页 W-977、W-978，主程序 CL-5） |
| 托盘「退出」 | `miyu core stop` | 退出码 0：核心停了，或者本来就没在跑；1：没停成，标准错误上一句「miyu: …」（核心 DT-4 下） |

`miyu web` 自己会把核心、网页软件拉起来（核心没在跑的起核心，网页软件没在跑的起 `miyu-web serve`），所以外壳不管谁先起。

### 在哪

| 文件 | 管什么 |
|---|---|
| `Cargo.toml`、`build.rs`、`tauri.conf.json` | 一个 crate；Tauri 的配置：不打页面进应用（没有 `frontendDist`，窗口开时现给网址），没有 `capabilities/`、`capabilities` 是空的（远程来源的页面没有任何权限），不开 `withGlobalTauri` |
| `src/main.rs` | 起来：单实例、窗口状态、对话框三个插件，菜单的事件，日志和数据目录，事件循环 |
| `src/log.rs` | 外壳自己的日志 |
| `src/menu.rs` | macOS 的应用菜单 |
| `src/program.rs` | 找主程序 `miyu`，调它、读一行 JSON、读退出码和标准错误 |
| `src/window.rs` | 开窗口、要网址、出错的对话框、导航和下载交给谁、藏起来、叫回来 |
| `src/download.rs` | 下载存成哪个文件（纯逻辑，单测） |
| `src/navigate.rs` | 导航、新窗口、下载交给谁（纯逻辑，单测） |
| `src/tray.rs` | 托盘图标和菜单 |
| `src/quit.rs` | 彻底退出：藏窗口、叫核心停、退 |
| `src/notify.rs`、`src/notify/`（一个网页视图一份） | 网页视图的通知权限 |
| `src/text.rs`、`resources/text/{zh,en,ja}.toml` | 给人看的字 |
| `icons/` | 应用图标、托盘图标（施工 DT-5；之前是原图缩的占位图标） |
| `.github/workflows/ci.yml` | 三平台：格式、clippy、测试、行数、正式版的程序（试验用） |

### 怎么走

1. **起来**：单实例（`tauri-plugin-single-instance`）。已经有一个在跑的，第二次开只叫那一个把窗口拿出来、放到最前，自己退出（退出码 0）。没有托盘的桌面（GNOME 默认不显示托盘）靠这一条叫回窗口：再点一次应用图标。
2. **找主程序**：先看环境变量 `MIYU_PROGRAM`（开发时指到别处，和网页软件找主程序同一个名字）；再看外壳自己所在的目录里有没有 `miyu`（Windows 是 `miyu.exe`）；都没有的照 `PATH` 找。安装包里三样放在同一个目录（「装在哪」）。找不到的弹一个系统对话框「找不到 Miyu 的主程序」，只有「退出」。
3. **开窗口**：先建窗口（标题 `Miyu`，大小、位置照上次的，`tauri-plugin-window-state`；第一次 1200×800 居中），网页视图先空着；调 `miyu web --print --format json --ticket`（不开控制台窗口：Windows 上 `CREATE_NO_WINDOW`；最多等 30 秒），拿到网址就导航过去。每次新开窗口现要一次，不存网址、不存票：票一次性，页面用票连上以后自己在这个网页视图里存设备令牌，刷新照它进。
   - 退出码非 0、超时、读不出那一行的：弹对话框，头一句「打不开 Miyu 的页面。」，下面是 `miyu` 标准错误的最后几行（照它的语言）或者超时、读不懂的一句，两个按钮「重试」「退出」。「退出」只退外壳，不叫核心停。
4. **网页视图的数据**（`localStorage` 里的设备令牌、缓存）放在外壳自己的数据目录（Tauri 的 `data_directory`，在应用标识 `io.github.miyu-agent.desktop` 的目录下），不放进 Miyu 的数据根；卸载外壳不动数据根。
5. **页面能碰到的外壳**：没有。不开 Tauri 的进程间通道给远程来源的页面（`capabilities` 里不列 `http://127.0.0.1`），页面里没有 `window.__TAURI__`；不装 opener 插件（它默认接管 `target=_blank` 的点击，DT-1 碰到过），开系统的浏览器直接用 `open` 这个 crate。页面要的东西都照浏览器的写法写，外壳在网页视图外面接住（下面 6 到 9）。要页面多给什么，找网页加在 `served.js` 里，不在外壳里开口子。
6. **导航**（`on_navigation`）：和页面同一个来源（同协议、同主机、同口子）的放行；`about:blank`、`blob:`、`data:` 照网页视图自己的规矩；别的一律不在窗口里开，交给系统的浏览器（`http`、`https`、`mailto`），别的协议不开。**新窗口**（`target=_blank`、`window.open`，`on_new_window`）同一条：同来源的也交系统浏览器（桌面端一个窗口一份状态，网页的「页面自己守的」第 4 条）。
7. **下载**（`on_download`）：照网页视图给的建议文件名存进系统的「下载」目录，重名的加「 (1)」「 (2)」；没有名字的叫 `download`。存完不弹东西（和浏览器一样，下载的东西在「下载」目录里）。
8. **通知**：页面自己照网页的规矩弹（网页 `web.md` 第 560 行一节，`src/host/notify.js`），外壳只管网页视图给不给这个权限：
   - Linux（WebKitGTK）：默认直接拒，外壳接 webkit2gtk 的 `permission-request`，通知的放行，别的（定位、摄像头、麦克风……）一律拒。
   - Windows（WebView2）、macOS（WKWebView，没有 `Notification` 这个接口）：施工 DT-3 照平台的试验包看，接不住的找网页在 `served.js` 里加。
   - 点了通知：窗口拿出来、放到最前（平台给这个事件的才做）。
9. **剪贴板**：照网页视图自己的（`navigator.clipboard`，DT-1 在 Linux 上试过能写）。macOS 上外壳要有带「编辑」那一栏的应用菜单，不然 `⌘C`、`⌘V` 在网页视图里不起作用（Tauri 默认的菜单有）。
10. **关窗口**：拦下关闭，窗口藏起来，网页视图留着：页面照旧连着核心、照旧收推送、照旧自己弹通知（DT-1：藏着时 `document.hidden` 为真，动画不跑）。macOS 上点程序坞的图标（`Reopen`）把窗口拿回来。
11. **托盘**：单色图标（macOS 照模板图）。菜单两项：「显示 Miyu」「退出」。Windows、macOS 左键点图标同「显示 Miyu」；Linux 只有菜单（托盘库在 Linux 上收不到图标的点击）。拿出窗口：显示、取消最小化、放到最前。
12. **彻底退出**（托盘「退出」；macOS 应用菜单的「退出 Miyu」`⌘Q` 同这一条。Tauri 默认菜单里的「退出」直接退外壳、不经这一条，所以 macOS 上外壳自己建应用菜单：「退出 Miyu」、系统的「编辑」一栏、「窗口」一栏的最小化和「关闭窗口」`⌘W`（同关窗口）；Linux、Windows 不设菜单栏）：先藏窗口、拿掉托盘图标，调 `miyu core stop`（它最多等 60 秒），再退出外壳。核心没停成（退出码 1）照样退，标准错误那句记进外壳的日志；网页软件不管，核心停了以后它照自己的规矩空闲 10 分钟退。浏览器里还开着的标签页重连时会把核心再拉起来，当成可以接受（DT-4 下）。
    - 关机、注销时系统叫外壳退：只退外壳，不叫核心停（系统会停它）。
13. **语言**：外壳自己的字（托盘菜单、对话框）照系统的语言，中文、日文、英文三份，别的用英文。不问核心的 `ui.language`：外壳不连核心，菜单就两三个词。
14. **日志**：外壳自己的一行一条写进它的数据目录里的 `desktop.log`，只记起来、要网址的耗时和结果、调 `miyu` 失败、退出；不记网址（里面有票）。

### 装在哪

三样放在一起，主程序找资源目录照核心的规矩（`MIYU_RESOURCES`，程序旁边的 `resources/`，上一级的 `share/miyu/`，上一级的 `Resources/miyu/`）：

| 平台 | 程序 | 资源目录（网页包在里面的 `packages/miyu-webui/`） |
|---|---|---|
| Linux（deb、rpm、AUR） | `/usr/bin/miyu-desktop`、`/usr/bin/miyu`、`/usr/bin/miyu-web` | `/usr/share/miyu/` |
| macOS（`.app`，dmg） | `Miyu.app/Contents/MacOS/` 下三个 | `Miyu.app/Contents/Resources/miyu/`（核心 DT-4 上） |
| Windows（NSIS） | 安装目录下三个 `.exe` | 安装目录下的 `resources/` |

Linux 的包写依赖 `webkit2gtk-4.1`、`libayatana-appindicator`、`bubblewrap`，不自带。核心、网页软件用哪一版钉在这个仓库里（施工 DT-6）。Linux 上已经有 `miyu`、`miyu-web` 的包时，桌面端的包也可以只带外壳、照版本依赖那两个包，施工 DT-6 定。

### 给人看的字

| 键 | 中文 |
|---|---|
| `tray.show` | 显示 Miyu |
| `tray.quit` | 退出 |
| `error.no_program` | 找不到 Miyu 的主程序。重新装一次 Miyu 试试。 |
| `error.open` | 打不开 Miyu 的页面。 |
| `error.timeout` | 等了 30 秒，Miyu 还没起来。 |
| `error.bad_reply` | Miyu 给的网址读不懂。 |
| `menu.quit` | 退出 Miyu（macOS 的应用菜单） |
| `menu.edit` | 编辑 |
| `menu.window` | 窗口 |
| `menu.close` | 关闭窗口 |
| `button.retry` | 重试 |
| `button.quit` | 退出 |

### 守着它的

| 测试 | 守什么 |
|---|---|
| `src/navigate/tests.rs` | 同来源放行（口子不同、`localhost` 换 `127.0.0.1` 的都不算同来源）；外链交系统浏览器；`javascript:`、`file:` 这些不开；新窗口一律交出去 |
| `src/program/tests.rs` | 找主程序的顺序；读那一行 JSON：多一个字段照读，少 `url` 的、不是 `http://127.0.0.1` 的算出错；退出码非 0 带回标准错误的最后几行；调的程序超时杀掉 |
| `src/download/tests.rs` | 重名加编号；没有名字的；名字里带路径分隔符、`..` 的只取最后一段 |
| `src/text/tests.rs` | 三份字的键一样多，没有空的；代码用的键都在；系统语言挑哪一份 |
| CI 三平台 | 编译、clippy `-D warnings`、测试；每个 `.rs` 不超过 500 行 |
| 手测清单（`docs/construction/` 里每步的验收） | 窗口、托盘、通知、下载、关窗口、退出：网页视图里的行为单测测不到，每个平台出试验包手测一遍 |
