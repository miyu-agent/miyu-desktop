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

### 手测（2026-10-31，Windows 11 虚拟机）

安装包：桌面 12e5570（核心 62f2e91c2，含核心 SW-4、SW-5、SW-6；网页 2029cf37）。`msiexec /qn` 覆盖安装，在交互会话里跑
`miyu ask --format jsonl --permission workspace`，让模型逐条原样跑命令；默认工作区的两条用 `-C` 指到一个会话的
`~\.miyu\workspace\<编号>`（网页、桌面开的会话用的就是这种目录）。

| 查什么 | 结果 |
|---|---|
| 验收 2：装好的 `resources\pwsh\pwsh.exe` 的版本 | 7.6.9 |
| 验收 3：沙盒里的 shell 用哪个 pwsh、`&&` | `C:\Program Files\Miyu\resources\pwsh\pwsh.exe`，7.6.9；`echo first && echo second` 两行 |
| 失败的命令（核心 SW-4） | 纯文本 `Get-Item: Cannot find path …`，没有 CLIXML |
| 默认工作区写、读、删（核心 SW-6） | PowerShell、包里的 Node（v22.23.3）都成；Node 的 `realpathSync` 不再 EPERM |
| 沙盒里读 `.miyu\run\token` | 读不到 |
| 沙盒里列 `.miyu` | 看得到名字（SW-6 的代价，核心定的） |

虚拟机上没有 Python，「Python 存文件」这一条用 PowerShell、Node 代替。从家目录起 `miyu ask`（不带 `-C`）时会话就在家目录、写被拒，已转给核心看是不是预期。

#### 复验核心 SW-7、CS-1（2026-11-01）

安装包 97ffc0a（核心 75bf68e90，网页 2029cf37），从 `C:\Users\shorin` 起 `miyu ask`（不带 `-C`）。会话在家目录干活，家目录里写、读、删都成，没有「范围过大」；令牌、`secrets.toml` 读不到；Edge 的用户数据藏起来了、系统凭据库列不了；`.miyu` 看得到名字；核心冷启动约 3 秒连上，没有告警。资源目录下的 `licenses/` 三份都在（DT-7）。

发现一个缺陷，转给核心：同一个会话里沙盒里的命令能往数据根 `~\.miyu` 新建文件（读被挡，写没挡）。没试私密目录和改写已有文件。

#### 撤回 SW-7、复验核心 SW-9（2026-11-02）

上面那个缺陷核心判为越权：`75bf68e90`～`d6ade212e` 的核心打出来的包作废（97ffc0a 那一版），核心 SW-8 撤回 SW-7，SW-9 改成藏起来的那一条落在能写的范围里时不挖洞、整条禁。在虚拟机上量了两组（测试目录，量完已删）：

- 藏起来的目录本身：新建、改写、追加、删、改名、建子目录、整个改名或删掉，MXC 全挡。洞出在挖洞：路上的祖先目录自己没藏，落在能写的里面就能往里加名字、能整个改名。
- 换钉 091fcfd2c（8484826）以后照挖洞的形状再量：数据根这一层新建、改名、删都被拒；明确列为能写的更深一层（工作区）照样能写。从家目录起的会话和 SW-7 以前一样：只有默认工作区能写，数据根写不进。
