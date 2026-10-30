; Windows 安装包的钩子（施工 DT-6；核心的主会话 2026-10-30 定，设计 11 A3）。安装包照 perMachine 提权跑，这几条都有管理员。
; 沙盒：装时 `miyu sandbox setup` 建沙盒用户，卸时 `miyu sandbox remove`。失败不挡安装（核心照「沙盒用不了」那一套照常跑），
; 原话留在安装日志里（nsExec::ExecToLog）。
; 装、卸之前先把在跑的停掉，不然程序被占着换不掉、删不掉（项目主人 2026-10-30 在虚拟机上碰到 miyu-web.exe 写不进去）：核心照
; `miyu core stop` 停；网页软件没有停的命令（它空闲 10 分钟自己退），和停不下来的核心一起，照路径只结束这个安装目录下的。

!macro MIYU_STOP_RUNNING
  IfFileExists "$INSTDIR\miyu.exe" 0 +4
    DetailPrint "miyu core stop"
    nsExec::ExecToLog '"$INSTDIR\miyu.exe" core stop'
    Pop $0
  DetailPrint "stop programs under $INSTDIR"
  ; 安装程序是 32 位的，拉起的 PowerShell 也是 32 位：Get-Process 读不到 64 位进程的 Path（空的），照 WMI 的 ExecutablePath 认
  ; （2026-10-30 虚拟机上第一版钩子什么都没停掉）。
  nsExec::ExecToLog `powershell -NoProfile -NonInteractive -Command "Get-CimInstance Win32_Process | Where-Object { $$_.ExecutablePath -like '$INSTDIR\*' } | ForEach-Object { 'stop ' + $$_.ProcessId + ' ' + $$_.ExecutablePath; Stop-Process -Id $$_.ProcessId -Force }"`
  Pop $0
!macroend

!macro NSIS_HOOK_PREINSTALL
  !insertmacro MIYU_STOP_RUNNING
!macroend

!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "miyu sandbox setup"
  nsExec::ExecToLog '"$INSTDIR\miyu.exe" sandbox setup'
  Pop $0
  DetailPrint "miyu sandbox setup: $0"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  !insertmacro MIYU_STOP_RUNNING
  DetailPrint "miyu sandbox remove"
  nsExec::ExecToLog '"$INSTDIR\miyu.exe" sandbox remove'
  Pop $0
  DetailPrint "miyu sandbox remove: $0"
!macroend
