## DT-8：通知

状态：在做（2026-10-31）。

### 目的

项目主人 2026-10-31 在三个平台上手测：系统通知一个都没弹。查下来三个平台各卡一处：Windows 的 WebView2 默认把网页通知的权限拒了（页面里 `Notification.permission` 是 `denied`、`notify_asked` 记成问过了）；macOS 的 WKWebView 根本没有 `Notification`；Linux 上外壳放行了权限，但那台 Arch 虚拟机没有通知服务（DankMaterialShell 没起）。窗口藏起来以后「她做完了、要你确认」靠通知告诉人，这是桌面端的本分。

### 蓝图改哪几节

- `docs/blueprint/desktop.md`「怎么走」第 5 条（页面能碰到的外壳：只有 `notify`）、第 8 条（三个平台各怎么弹）。

### 要拍板的

- 走外壳的替身（项目主人 2026-10-31 定）：页面照旧 `new Notification(…)`，Windows、macOS 上外壳放一个替身，转给系统的通知。代价是给页面开一个只能弹通知的口子，和原来「页面碰不到外壳」稍有出入。没选「各平台各自放行」：Windows 要写不安全的 COM 调用，macOS 还是弹不了。

### 不做什么

- Linux 不换：WebKitGTK 自己的通知能弹、点了能拿回窗口（DT-3 手测过），没有通知服务的桌面照旧弹不出来。
- 同一个 `tag` 换掉上一条、图标、按钮：第一版不做。

### 验收

1. 单测：页面那份能力只有 `allow-notify`、只给 `http://127.0.0.1:*`、只在 Windows、macOS；`tauri.conf.json` 只打开这一份、不开 `withGlobalTauri`；替身只调 `notify`。
2. 三台 CI 编译、clippy、测试过（macOS 那一半本机编不了）。
3. Windows、macOS 上手测：窗口藏着时她回完一轮弹系统通知；点了窗口回来；页面里 `Notification.permission` 是 `granted`。

### 风险

- Windows 上没经 MSI 的快捷方式起（直接双击 exe）时，toast 认不出这个应用标识，可能弹不出来。
- macOS 上通知照包的标识走，第一次会问人，拒了以后要在「系统设置 → 通知」里开。
