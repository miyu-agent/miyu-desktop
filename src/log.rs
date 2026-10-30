//! 外壳自己的日志（蓝图「怎么走」第 14 条）：一行一条，写进它的数据目录里的 `desktop.log`。只记起来、要网址的耗时和
//! 结果、调 `miyu` 失败、退出；不记网址（里面有票）。还没定下目录之前、写不进去的，写到标准错误上。

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static FILE: OnceLock<PathBuf> = OnceLock::new();
static WRITING: Mutex<()> = Mutex::new(());

/// 定下日志文件。只认第一次。
pub fn start(path: PathBuf) {
    if let Err(path) = FILE.set(path) {
        line(&format!(
            "log already started, not moving to {}",
            path.display()
        ));
    }
}

/// 记一行：前面是 Unix 时间（秒，三位小数）。
pub fn line(message: &str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let entry = format!("{}.{:03} {message}\n", now.as_secs(), now.subsec_millis());
    let Some(path) = FILE.get() else {
        eprint!("{entry}");
        return;
    };
    let _guard = WRITING
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let written = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut file| file.write_all(entry.as_bytes()));
    if let Err(error) = written {
        eprintln!("{entry}(not written to {}: {error})", path.display());
    }
}
