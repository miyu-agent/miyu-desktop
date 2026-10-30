//! 下载存在哪（蓝图「怎么走」第 7 条）：「下载」目录里，照网页视图建议的名字，重名的加「 (1)」「 (2)」。

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// 没有名字的下载叫这个。
const UNNAMED: &str = "download";

/// 存进 `dir` 的哪个文件：名字只取 `suggested` 的最后一段，空的、`.`、`..` 换成 `download`；`taken` 说已经有了的往后加编号。
pub fn destination(
    dir: &Path,
    suggested: Option<&OsStr>,
    taken: impl Fn(&Path) -> bool,
) -> PathBuf {
    let name = suggested
        .and_then(|s| Path::new(s).file_name())
        .filter(|n| !n.is_empty())
        .unwrap_or(OsStr::new(UNNAMED));
    let first = dir.join(name);
    if !taken(&first) {
        return first;
    }
    let named = Path::new(name);
    let stem = named.file_stem().unwrap_or(name).to_string_lossy();
    let extension = named
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()));
    (1u32..)
        .map(|n| {
            dir.join(format!(
                "{stem} ({n}){}",
                extension.as_deref().unwrap_or("")
            ))
        })
        .find(|path| !taken(path))
        .unwrap_or(first)
}

#[cfg(test)]
mod tests;
