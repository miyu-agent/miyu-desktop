## DT-6：打包和发版

状态：在做（2026-10-30）。

### 目的

出三平台的安装包，装上就能用：Linux 的 deb、rpm、AUR，Windows 的 NSIS，macOS 的 dmg。包里带外壳、核心（三个程序加资源目录）和网页软件（项目主人 2026-10-29 定，`README.md` 第一节第 5、6 条）。核心没有出安装包的流水线，这里照钉的提交号自己编。

### 蓝图改哪几节

- `docs/blueprint/desktop.md`「装在哪」整节重写（程序、资源目录、网页包的样子、三个平台各自的讲究、钉版本和流水线）；「在哪」加打包的文件。

### 要拍板的

- 带哪几个程序、资源目录、编法、沙盒：核心的主会话 2026-10-30 定（`miyu`、`miyu-sandbox`、`miyu-extract`，整份 `resources/`，Windows 装时跑 `miyu sandbox setup`、卸时 `remove`，失败不挡安装）。
- 网页包在出厂层的样子：网页的会话 2026-10-30 定（照 `tools/pack-web.py`，清单补 version、不写 targets，程序放 `bin/`）；网页照它的 main 钉，不照核心 `heads.toml` 里改名前的那一版。
- macOS 做本机的临时签名：不签 Apple 芯片上跑不起来；这不是项目主人说的「签名」（开发者证书、公证），第一次打开照样要放行。
- AUR：出填好的 PKGBUILD 传进发版，推到 AUR 要项目主人的账号，由项目主人做。

### 不做什么

- 开发者证书签名、公证、自动更新（第一版不做）。
- AppImage、Flatpak。
- 核心自己的安装包：核心的事。

### 验收

1. `packaging/stage.py` 照钉的两个提交号搭出内容；本机 Linux 打出 deb、rpm，拆开看程序、资源目录、网页包、依赖都对。
2. 本机装 deb（或者拆到一个临时根里照样起）：外壳起来、免密进来、托盘「退出」核心停。
3. 三台的「打包」流水线绿，收到 deb、rpm、dmg、setup.exe、PKGBUILD；包里的 `miyu --version` 起得来。
4. 出包以后 macOS、Windows 上手测窗口、托盘、通知、菜单、`⌘Q`、安装卸载（找项目主人，或者虚拟机）。

### 风险

- 网页的程序装在 `/usr/share/miyu/packages/miyu-webui/bin/` 下：能用，但 deb 的规矩上 `/usr/share` 里不放程序；有人挑再挪成放在 `miyu` 旁边（核心两处都找）。
- Windows 的 WebView2：Windows 10 没装的，NSIS 默认下载引导程序装；离线装不上。

### Windows 手测（项目主人的 Windows 11 虚拟机，2026-10-30）

| 项 | 结论 | 依据 |
|---|---|---|
| 装 | 行 | NSIS 安装包（17 MB）装进 `C:\Program Files\Miyu\`，四个程序和 `resources\` 都在；开始菜单、桌面的「Miyu」快捷方式 |
| 第一次开窗口拿不到网址 | 修了 | 弹「Miyu 给的网址读不懂」：`miyu web` 顺带拉起的核心、网页软件继承了外壳读输出的管道，外壳等读到头等不到。改成读到一块收一块（f45bf97），之后免密进到欢迎屏 |
| 安装程序没有图标 | 修了 | 用的是 NSIS 默认的；`installerIcon` 换成 `icons/icon.ico` |
| 启动时黑窗口一闪 | 核心的事 | 进程追踪：核心（没有控制台）拉起 `miyu-sandbox.exe` 没带 `CREATE_NO_WINDOW`，Windows 11 新开控制台交给 Windows Terminal。外壳调 `miyu` 带了，不弹。已交核心的主会话 |
| 黑屏、刚打开时卡 | 别家的程序 | WebView2 的主进程崩在 `RTSSHooks64.dll`（RivaTuner Statistics Server 的钩子，注入每个用显卡画的程序）；页面文件 276 个 0.8 秒送完，不是网页软件慢；GPU 是硬件渲染（NVIDIA）。项目主人关掉 RTSS 再测 |

- 网页软件给页面文件只回 `Cache-Control: no-cache`、不带 ETag，每次开窗口整套重下：转给网页的会话看要不要加。
- WebView2 的进程被别的程序弄崩时窗口只剩黑的、外壳不知道：Tauri 不给 WebView2 的 `ProcessFailed`，要接得写不安全的 COM 调用（仓库 `forbid(unsafe_code)`）。先记着；碰到的多了再找 Tauri 或者照 wry 的接口加。
