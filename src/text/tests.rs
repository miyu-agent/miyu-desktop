use super::*;

const ALL: [Lang; 3] = [Lang::Zh, Lang::En, Lang::Ja];

fn keys(lang: Lang) -> Vec<String> {
    let lines: BTreeMap<String, String> = toml::from_str(lang.source()).unwrap();
    lines.into_keys().collect()
}

#[test]
fn all_three_have_the_same_keys() {
    for lang in ALL {
        assert_eq!(keys(lang), keys(Lang::Zh), "{lang:?}");
    }
}

#[test]
fn no_line_is_empty() {
    for lang in ALL {
        let text = Text::of(lang);
        for key in keys(lang) {
            assert!(!text.get(&key).trim().is_empty(), "{lang:?} {key}");
            assert_ne!(text.get(&key), key, "{lang:?} {key}");
        }
    }
}

#[test]
fn the_keys_the_code_uses_are_there() {
    let zh = keys(Lang::Zh);
    for key in crate::text::KEYS {
        assert!(zh.iter().any(|k| k == key), "{key}");
    }
}

#[test]
fn the_language_follows_the_locale() {
    assert_eq!(Lang::of_locale("zh-CN"), Lang::Zh);
    assert_eq!(Lang::of_locale("zh_TW.UTF-8"), Lang::Zh);
    assert_eq!(Lang::of_locale("ja-JP"), Lang::Ja);
    assert_eq!(Lang::of_locale("en-US"), Lang::En);
    assert_eq!(Lang::of_locale("de-DE"), Lang::En);
    assert_eq!(Lang::of_locale(""), Lang::En);
}

#[test]
fn a_missing_key_falls_back_to_its_name() {
    assert_eq!(Text::of(Lang::En).get("no.such.key"), "no.such.key");
}
