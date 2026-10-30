## DT-9：Windows 安装包带 PowerShell 7

状态：在做（2026-10-31）。

### 目的

Windows 上模型照 bash、PowerShell 7 的习惯写 `cmd1 && cmd2`，系统自带的 Windows PowerShell 5.1 不认（「标记“&&”不是此版本中的有效语句分隔符」），一句命令整个失败。项目主人 2026-10-31 定：不在结果末尾补提示，安装包直接带官方的 PowerShell 7（zip 版）；核心先找资源目录里的 `pwsh\pwsh.exe`，没有再找 PATH 里的，最后退回 5.1（核心 SW-5）。

### 蓝图改哪几节

- `docs/blueprint/desktop.md`「装在哪」：Windows 的资源目录多一个 `pwsh\`。

### 要拍板的

- 版本：当前 LTS 的最新补丁 7.6.9（2026-10-25），钉在 `pins.toml`，两个 zip 的校验值照官方发版的 `hashes.sha256`。x64 机器用 win-x64，ARM64 用 win-arm64（照编出来的三元组挑）。
- 安装包变大：PowerShell 解开约 250 MB，MSI 从 25 MB 涨到一百多 MB。项目主人定「安装包里带 zip 版」时接受了。

### 不做什么

- 不装成系统的 PowerShell（不进 PATH、不写注册表、不碰 Microsoft Store 那一份），只放在 Miyu 的资源目录里给核心用。
- 第三方许可证：PowerShell 是 MIT，zip 里自带 LICENSE.txt、ThirdPartyNotices.txt，原样留在 `resources\pwsh\` 里；并进安装包的第三方许可证清单随 DT-7。

### 验收

1. 打包流水线下载官方 zip、核对校验值、解进 `dist/resources/pwsh/`，校验值不对就停。
2. 装好以后 `[INSTALLDIR]resources\pwsh\pwsh.exe -NoProfile -Command '$PSVersionTable.PSVersion'` 是 7.6.9。
3. 经 `miyu-sandbox` 在沙盒里起这个 pwsh，`&&` 能用。
4. MSI 装、卸之前停进程的名单有 `pwsh`（只停安装目录下的）。

### 风险

- 每次打 Windows 包都从 GitHub 下 110 MB：GitHub 抽风时打包失败，重跑就行。
