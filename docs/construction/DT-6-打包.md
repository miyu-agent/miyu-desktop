## DT-6：打包和发版

状态：已完成（2026-10-31）。三台的「打包」流水线出得了 deb、rpm、PKGBUILD、dmg、64 位 MSI；Linux（Arch 虚拟机装 pacman 包）、Windows（Win11 虚拟机装 MSI）项目主人手测过。macOS 只出包、拆开看过，没在 Mac 上手测；Windows 上剩下的几件是核心、网页的事（见「Windows 手测」末尾），跟着它们换钉。

### 目的

出三平台的安装包，装上就能用：Linux 的 deb、rpm、AUR，Windows 的 MSI（64 位；原来是 NSIS，2026-10-30 项目主人要 64 位的安装程序，换了），macOS 的 dmg。包里带外壳、核心（三个程序加资源目录）和网页软件（项目主人 2026-10-29 定，`README.md` 第一节第 5、6 条）。核心没有出安装包的流水线，这里照钉的提交号自己编。

### 蓝图改哪几节

- `docs/blueprint/desktop.md`「装在哪」整节重写（程序、资源目录、网页包的样子、三个平台各自的讲究、钉版本和流水线）；「在哪」加打包的文件。

### 要拍板的

- 带哪几个程序、资源目录、编法、沙盒：核心的主会话 2026-10-30 定（`miyu`、`miyu-sandbox`、`miyu-extract`，整份 `resources/`）。原来定 Windows 装时跑 `miyu sandbox setup`、卸时 `remove`；同一天核心的主会话又说 MXC 跑命令不用沙盒用户，安装包里不跑（setup 会往跑它的人的数据根写东西，MSI 提权的步骤是 SYSTEM，写错地方）。
- Windows 换 64 位的 MSI（项目主人 2026-10-30：「什么年代了，还在用32位」）：NSIS 只有 32 位，Tauri 里 64 位的安装包只有 WiX 出的 MSI。一个 MSI 只带一种界面语言，出中、英、日三个。
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
| 覆盖安装写不进 `miyu-web.exe` | 修了 | 上一次的网页软件还在跑（它空闲 10 分钟才退，没有停的命令）。钩子装、卸之前照路径结束这个安装目录下还在跑的 `miyu-web`、`miyu`、`miyu-extract`、`miyu-sandbox`（核心先照 `miyu core stop` 停） |
| 启动时黑窗口一闪 | 核心的事 | 进程追踪：核心（没有控制台）拉起 `miyu-sandbox.exe` 没带 `CREATE_NO_WINDOW`，Windows 11 新开控制台交给 Windows Terminal。外壳调 `miyu` 带了，不弹。核心修了（NW-1，f17fdbc5c），换钉以后项目主人复测不闪了 |
| 黑屏、刚打开时卡 | 别家的程序 | WebView2 的主进程崩在 `RTSSHooks64.dll`（RivaTuner Statistics Server 的钩子，注入每个用显卡画的程序）；页面文件 276 个 0.8 秒送完，不是网页软件慢；GPU 是硬件渲染（NVIDIA）。项目主人关掉 RTSS 再测：不卡、不黑屏 |
| 安装程序是 32 位 | 换了 | NSIS 只有 32 位（它拉起的 PowerShell 也是 32 位，读不到 64 位进程的路径，第一版停进程的钩子因此什么都没停掉）。换成 WiX 出的 64 位 MSI，停进程用 `System64Folder` 下的 PowerShell |
| 聚焦窗口就连闪黑框 | 网页修了 | 进程追踪：页面每次拿回焦点问壁纸，网页软件拉 `reg.exe` 没带 `CREATE_NO_WINDOW`，30 秒 12 次。网页 W-1016（dabb37c4）：带上标志、记着壁纸的回答、一秒内只问一次 |
| 「设置工作区」说找不到目录 | 核心、网页修了 | 握手的 `host.workspace` 带 `\\?\` 前缀（核心 WP-1，eb55db742 去掉）；网页兜一层（W-1011）。调试口子里调 `/pick-dir` 回 200、原生的选目录对话框弹得出来 |
| 沙盒里的命令 125 | 核心修了 | 禁止路径里有 `NTUSER.DAT`、`UsrClass.dat`，登录着时系统独占，`CreateProcessSecurityEnvironment` 报 0x80070020；核心 SW-1（eb55db742）跳过被独占的文件 |
| 沙盒里 PowerShell 进受限语言模式 | 核心的事，在修 | 出厂私密清单整个禁了 `AppData\Local\Packages`，容器自己的 TEMP 在它下面；PowerShell 写不了测试脚本就进 ConstrainedLanguage，核心开头那句 `[Console]::OutputEncoding = …` 跟着失败。外面设的 TEMP 被 MXC 改回容器自己的。另外 WMI（`Get-CimInstance`）在容器里本来就被挡，放不放核心问项目主人 |
| 很多动画不动 | 这台机器的设置 | 交互会话里「动画效果」是关的（`SPI_GETCLIENTAREAANIMATION=False`），WebView2 报 `prefers-reduced-motion: reduce`，页面照规矩停了动画；调试口子里核实 |
| 安装程序打不开 | 窗口被压在后面 | 三次双击起了三个 `msiexec /i`、都没露面；结束掉以后从交互会话重开，1 秒到欢迎页 |

- 网页软件给页面文件只回 `Cache-Control: no-cache`、不带 ETag，每次开窗口整套重下：网页的会话加了 ETag、304（W-1006，网页 0e210c97 起）。
- WebView2 的进程被别的程序弄崩时窗口只剩黑的、外壳不知道：Tauri 不给 WebView2 的 `ProcessFailed`，要接得写不安全的 COM 调用（仓库 `forbid(unsafe_code)`）。先记着；碰到的多了再找 Tauri 或者照 wry 的接口加。

### Arch 手测（项目主人的 Arch 虚拟机，niri，WebKitGTK 2.54.2，2026-10-30、31）

| 项 | 结论 | 依据 |
|---|---|---|
| 装 | 行 | 流水线出的 PKGBUILD 在本机 makepkg 成 `miyu-desktop-bin`（拆 deb），`pacman -U` 装上：依赖 webkit2gtk-4.1、gtk3、libayatana-appindicator、bubblewrap，provides、conflicts `miyu`，四个程序、`/usr/share/miyu` 1186 个文件、桌面文件都在 |
| 拿网址、页面 | 行 | 装好的 `miyu web` 19 毫秒；网页软件从 `/usr/share/miyu/packages/miyu-webui/bin/` 起；主人在 niri 里开窗口、托盘、退出都过了 |
| 外链、软件后台的 ↗ 打不开 | 这台机器的设置 | 默认浏览器是 `dms-open.desktop`（`dms open`，要 DankMaterialShell 在跑，这个会话里没起），`xdg-open` 落空；项目主人让改成 Firefox（`xdg-settings set default-web-browser firefox.desktop`）以后好了 |
| 输入框下沿一条细线 | 网页修了 | WebKitGTK 画的横向滚动条；网页 W-1022（1b918348）加 `overflow-x: hidden`、`appearance: none`，复测没了 |

- 这台虚拟机上原来就装着一份 `~/.local` 下的 Miyu（核心开着、带 IM 桥）。测的时候我跑 `miyu core stop` 停了它约 25 秒，照它自己的程序拉回来了；以后在共用的机器上先看清是谁的核心再动。
- 交给系统浏览器成没成，外壳现在不知道（`open::that_detached` 只管拉起）。要不要等 `xdg-open` 跑完、失败了记一行，留给以后。

### 留给后面的

- macOS 上手测：窗口、托盘（模板图）、通知、`⌘Q`、dmg 装、第一次打开放行。
- Windows：沙盒里失败时还是 CLIXML、CIM 失败没接上「沙盒挡着 WMI」那句、受保护路径列表混着 Linux 的出厂项（核心）；路径照 Windows 的写法显示、添加框的例子（网页）。修了换钉。
- 发版：打 v 开头的 tag 出预发布、AUR 由项目主人推。
