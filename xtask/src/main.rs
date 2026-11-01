//! 桌面端的门禁里 cargo 自己不管的几项（施工 DT-7）。格式、clippy、测试照旧在 CI 里直接跑。
//!
//! - `cargo xtask check`：「许可证」一项。三个平台上编进外壳的第三方依赖，许可证都能和 AGPL-3.0-or-later 合在一起发
//!   （`licenses.rs`）；第三方许可证清单 `THIRD-PARTY-LICENSES.txt` 是最新的（`notices.rs`）。
//! - `cargo xtask notices`：重新生成那份清单。升了依赖、加了依赖的那一步跟着跑，和代码进同一个提交。

mod licenses;
mod notices;

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    match std::env::args().nth(1).as_deref() {
        Some("check") => {
            let mut problems = licenses::check(&root(), &cargo);
            problems.extend(notices::check(&root(), &cargo));
            if problems.is_empty() {
                println!("许可证：没有问题");
                return ExitCode::SUCCESS;
            }
            problems.iter().for_each(|problem| println!("{problem}"));
            println!("许可证：{} 处", problems.len());
            ExitCode::FAILURE
        }
        Some("notices") => match notices::write(&root(), &cargo) {
            Ok(()) => {
                println!("写好了 {}", notices::PATH);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!("用法：cargo xtask check | cargo xtask notices");
            ExitCode::from(2)
        }
    }
}

/// 仓库根：这个小程序的上一级。
fn root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.parent().map(PathBuf::from).unwrap_or(here)
}
