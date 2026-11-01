## DT-7：许可证门禁和随包的许可证清单

状态：做完（2026-11-01），等 CI 和三台打包。

### 目的

安装包把外壳、核心、网页软件打在一起发，MIT、BSD、Apache 都要求分发时附上版权和许可证原文，MPL 还要告诉人源码从哪拿（发布前审查 2026-10-30 第 9、17 条）。核心 AU-2（cc065472d）定了做法：xtask 自己写，清单生成好提交进仓库，门禁查它是不是最新的。外壳照搬一份，安装包把三份都带上。

### 蓝图改哪几节

- `docs/blueprint/desktop.md`「在哪」：多一行 `xtask/`、`.cargo/config.toml`、`THIRD-PARTY-LICENSES.txt`；CI 那一行加许可证。
- 同一页「装在哪」：资源目录下的 `licenses/`。

### 做了什么

1. **门禁**：`xtask/src/licenses.rs` 照 miyu-im 的（那份照核心的）。平台换成出安装包的三个（Linux x64、Apple 芯片的 macOS、Windows x64），放行的名单同核心（AGPL-3.0-or-later 能合在一起发的那些，GPL-3.0-or-later 也在：两份第 13 条互相允许）。核心、网页软件外壳不经 crate 用，不在依赖图里，不用放行 AGPL 的 git 依赖。
2. **清单**：`xtask/src/notices.rs` 照核心的。三处不同：
   - 从外壳走，不从门禁的小程序走（`xtask` 在工作区里，它的依赖不编进安装包）。
   - 多一份标准原文 MPL-2.0：Tauri 带进来的 `selectors 0.38.0`（Servo 的）没带许可证文件，原文照 `option-ext` 带的那份（Mozilla 现在发的，https 的网址）。
   - MPL-2.0 的包段前那一行后面写 `source code: https://crates.io/crates/<名字>/<版本>`。2026-11-01 有五个：`cssparser`、`cssparser-macros`、`dtoa-short`、`option-ext`、`selectors`。
   - 没有「资源里的第三方数据」：外壳的资源（图标、字、通知的替身）都是自己的。
   2026-11-01 生成出来 213 段，约 690 KB。
3. **工作区**：根上的 `Cargo.toml` 加 `[workspace]`（成员 `xtask`），lints 挪到 `[workspace.lints]` 两边共用；`.cargo/config.toml` 设 `cargo xtask` 别名。
4. **CI**：clippy、测试带 `--workspace`；行数也数 `xtask/src`；Linux 上跑 `cargo xtask check`（`cargo metadata --filter-platform` 不用装别的目标）。
5. **安装包**：`packaging/stage.py` 的 `stage_licenses` 把三份放进 `dist/resources/licenses/`：`miyu-desktop/`（这个仓库的 `LICENSE`、清单）、`miyu/`（核心检出的 `LICENSE`、`LICENSES/`、清单）、`miyu-web/`（网页检出的 `LICENSE`、`miyu-webui/THIRD-PARTY-LICENSES.txt`），加一份 `README.txt`（Windows 的另说 `pwsh\` 里有 PowerShell 自己的两份）。缺一份就停。

### 要拍板的

- 放在资源目录的 `licenses/`，不放安装目录的根上：三个平台的包都把资源目录整份带上，放这里不用分平台写打包配置；Linux 的 deb、rpm 落在 `/usr/share/miyu/licenses/`。
- 网页的清单在网页包里已经有一份，`licenses/miyu-web/` 再放一份（约几百 KB）：看许可证的人一个目录看全。

### 不做什么

- 不改核心、网页的清单：它们各自生成、各自的门禁守着，这里只拷。
- Tauri 带进来的几个不再维护、unsound 的提醒，不在这一步处理，跟着 Tauri 升级（2026-11-01 核对，都只在 Linux 的依赖图里，GTK 那一串带进来的）：`proc-macro-error 1.0.4`、`zbus_names 4.3.5`、`zvariant 5.15.1`、`zvariant_derive 5.15.1`、`glib 0.18.5`。许可证都能用，门禁不拦。

### 验收

1. `cargo xtask check` 过：三个平台的依赖许可证都能用，清单是最新的；改一个依赖不重新生成，它报「不是最新的」。
2. xtask 的测试：SPDX 表达式、每个包只报一次、只看依赖图里的第三方包、只顺着正常依赖走且不从 `xtask` 走、读许可证文件、没带文件的用标准原文（含 MPL-2.0）、报不出来的报错、同样的原文合一段、MPL 的写源码地址。
3. `stage_licenses` 照两个钉住的提交试过：三份齐全、Windows 多一句 PowerShell、缺一份就停。
4. 三台打包跑通，装好的包里 `resources/licenses/` 有三个目录。

### 风险

- 升 Tauri、加依赖忘了跑 `cargo xtask notices`：CI 的许可证一步会红，照提示跑一遍提交上来。
