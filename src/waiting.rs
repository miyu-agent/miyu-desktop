//! 等待页（蓝图「怎么走」第 3 条）：要网址的那一会儿（刚装完的 Windows 上能等几十秒）窗口里先放的一页。网页包带着
//! `loading.html`（网页 W-1032：欢迎页那个能动的字符画剪影，一个文件自带一切，项目主人 2026-10-31 要的），外壳照核心找
//! 资源目录的规矩读它，把 `{{starting}}` 换成这种语言的「正在启动 Miyu…」；读不到的（开发时、旧的网页包）放一行字。
//! 都作为 `data:` 页面开（导航放行，`navigate.rs`）：网页软件还没起来，不能从它那里取。

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// 网页包里那一页，相对资源目录。
const LOADING: &str = "packages/miyu-webui/loading.html";
/// 那一页里换成「正在启动 Miyu…」的占位。
const SLOT: &str = "{{starting}}";

/// 等待页的 `data:` 网址。
pub fn page(words: &str) -> String {
    let html = loading()
        .filter(|html| html.contains(SLOT))
        .map(|html| html.replacen(SLOT, &escape(words), 1))
        .unwrap_or_else(|| plain(words));
    format!(
        "data:text/html;charset=utf-8;base64,{}",
        base64(html.as_bytes())
    )
}

/// 照这个进程的环境找网页包里的那一页。
fn loading() -> Option<String> {
    let own = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf));
    let dir = resources(
        std::env::var_os("MIYU_RESOURCES"),
        own.as_deref(),
        Path::is_dir,
    )?;
    std::fs::read_to_string(dir.join(LOADING)).ok()
}

/// 资源目录：照核心的规矩（核心 `docs/blueprint/store/resources.md`）。`MIYU_RESOURCES` 写了的用它；不然程序旁边的
/// `resources/`、上一级的 `share/miyu/`、上一级的 `Resources/miyu/`，哪个在用哪个。
pub fn resources(
    given: Option<OsString>,
    own_dir: Option<&Path>,
    exists: impl Fn(&Path) -> bool,
) -> Option<PathBuf> {
    if let Some(given) = given.filter(|g| !g.is_empty()) {
        return Some(PathBuf::from(given));
    }
    let own = own_dir?;
    let up = own.parent();
    [
        Some(own.join("resources")),
        up.map(|u| u.join("share").join("miyu")),
        up.map(|u| u.join("Resources").join("miyu")),
    ]
    .into_iter()
    .flatten()
    .find(|p| exists(p))
}

/// 读不到网页包那一页时放的：一句话，跟着系统的明暗。
fn plain(words: &str) -> String {
    format!(
        "<!doctype html><meta charset=\"utf-8\"><style>html,body{{margin:0;height:100%}}body{{display:flex;align-items:center;\
         justify-content:center;font:15px system-ui,sans-serif;color:#6b6b6b;background:#faf8f5}}@media(prefers-color-scheme:dark)\
         {{body{{color:#a0a0a8;background:#1c1c22}}}}</style><p>{}</p>",
        escape(words)
    )
}

/// 放进 HTML 的字：`&`、`<`、`>`、`"` 换掉。
fn escape(words: &str) -> String {
    words
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// 标准的 base64（带 `=` 补齐）：`data:` 网址里比百分号编码短。
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..4 {
            if i <= chunk.len() {
                let index = usize::try_from((n >> (18 - 6 * i)) & 63).unwrap_or(0);
                out.push(char::from(ALPHABET[index]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests;
