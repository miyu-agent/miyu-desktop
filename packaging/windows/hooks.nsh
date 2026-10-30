; Windows 安装包的钩子（施工 DT-6；核心的主会话 2026-10-30 定，设计 11 A3）。安装包照 perMachine 提权跑，这几条都有管理员。
; 沙盒：装时 `miyu sandbox setup` 建沙盒用户，卸时 `miyu sandbox remove`。失败不挡安装（核心照「沙盒用不了」那一套照常跑），
; 原话留在安装日志里（nsExec::ExecToLog）。装、卸之前先叫在跑的核心停（`miyu core stop`），不然 miyu.exe 被占着换不掉。

!macro NSIS_HOOK_PREINSTALL
  IfFileExists "$INSTDIR\miyu.exe" 0 +4
    DetailPrint "miyu core stop"
    nsExec::ExecToLog '"$INSTDIR\miyu.exe" core stop'
    Pop $0
!macroend

!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "miyu sandbox setup"
  nsExec::ExecToLog '"$INSTDIR\miyu.exe" sandbox setup'
  Pop $0
  DetailPrint "miyu sandbox setup: $0"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  DetailPrint "miyu core stop"
  nsExec::ExecToLog '"$INSTDIR\miyu.exe" core stop'
  Pop $0
  DetailPrint "miyu sandbox remove"
  nsExec::ExecToLog '"$INSTDIR\miyu.exe" sandbox remove'
  Pop $0
  DetailPrint "miyu sandbox remove: $0"
!macroend
