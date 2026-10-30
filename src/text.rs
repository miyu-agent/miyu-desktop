//! 外壳自己给人看的字（蓝图「怎么走」第 13 条）：托盘菜单、应用菜单、对话框。照系统的语言挑一份，中文、日文、英文，
//! 别的用英文；不问核心的 `ui.language`（外壳不连核心）。三份住在 `resources/text/`，编进程序。

use std::collections::BTreeMap;

/// 代码里用的键。
pub mod key {
    pub const TRAY_SHOW: &str = "tray.show";
    pub const TRAY_QUIT: &str = "tray.quit";
    #[cfg_attr(
        not(target_os = "macos"),
        allow(dead_code, reason = "只有 macOS 的应用菜单用")
    )]
    pub const MENU_QUIT: &str = "menu.quit";
    #[cfg_attr(
        not(target_os = "macos"),
        allow(dead_code, reason = "只有 macOS 的应用菜单用")
    )]
    pub const MENU_EDIT: &str = "menu.edit";
    #[cfg_attr(
        not(target_os = "macos"),
        allow(dead_code, reason = "只有 macOS 的应用菜单用")
    )]
    pub const MENU_WINDOW: &str = "menu.window";
    #[cfg_attr(
        not(target_os = "macos"),
        allow(dead_code, reason = "只有 macOS 的应用菜单用")
    )]
    pub const MENU_CLOSE: &str = "menu.close";
    pub const NO_PROGRAM: &str = "error.no_program";
    pub const OPEN: &str = "error.open";
    pub const TIMEOUT: &str = "error.timeout";
    pub const BAD_REPLY: &str = "error.bad_reply";
    pub const RETRY: &str = "button.retry";
    pub const QUIT: &str = "button.quit";
    pub const STARTING: &str = "starting";
}

/// 上面每一个键，测试查三份里都有。
#[cfg(test)]
const KEYS: [&str; 13] = [
    key::TRAY_SHOW,
    key::TRAY_QUIT,
    key::MENU_QUIT,
    key::MENU_EDIT,
    key::MENU_WINDOW,
    key::MENU_CLOSE,
    key::NO_PROGRAM,
    key::OPEN,
    key::TIMEOUT,
    key::BAD_REPLY,
    key::RETRY,
    key::QUIT,
    key::STARTING,
];

/// 一种语言。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Zh,
    En,
    Ja,
}

impl Lang {
    /// 照系统的语言标签挑：`zh*` 中文，`ja*` 日文，别的英文。
    pub fn of_locale(locale: &str) -> Self {
        let lower = locale.to_ascii_lowercase();
        if lower.starts_with("zh") {
            Self::Zh
        } else if lower.starts_with("ja") {
            Self::Ja
        } else {
            Self::En
        }
    }

    fn source(self) -> &'static str {
        match self {
            Self::Zh => include_str!("../resources/text/zh.toml"),
            Self::En => include_str!("../resources/text/en.toml"),
            Self::Ja => include_str!("../resources/text/ja.toml"),
        }
    }
}

/// 一份字：键到这种语言的一句。
#[derive(Debug)]
pub struct Text {
    lines: BTreeMap<String, String>,
}

impl Text {
    /// 照这个进程的系统语言挑一份。
    pub fn system() -> Self {
        Self::of(Lang::of_locale(
            &sys_locale::get_locale().unwrap_or_default(),
        ))
    }

    /// 这种语言的那一份。读不出来的（不会：测试查过三份）记一行日志，每个键照键名给。
    pub fn of(lang: Lang) -> Self {
        match toml::from_str(lang.source()) {
            Ok(lines) => Self { lines },
            Err(error) => {
                crate::log::line(&format!("text for {lang:?} is broken: {error}"));
                Self {
                    lines: BTreeMap::new(),
                }
            }
        }
    }

    /// 这个键的一句；没有的照键名给。
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.lines.get(key).map_or(key, String::as_str)
    }
}

#[cfg(test)]
mod tests;
