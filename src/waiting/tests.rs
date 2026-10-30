use super::*;

#[test]
fn base64_matches_the_standard() {
    assert_eq!(base64(b""), "");
    assert_eq!(base64(b"f"), "Zg==");
    assert_eq!(base64(b"fo"), "Zm8=");
    assert_eq!(base64(b"foo"), "Zm9v");
    assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    assert_eq!(base64("正".as_bytes()), "5q2j");
}

#[test]
fn the_environment_variable_wins() {
    let got = resources(
        Some("/dev/res".into()),
        Some(Path::new("/opt/m/bin")),
        |_| true,
    );
    assert_eq!(got, Some(PathBuf::from("/dev/res")));
}

#[test]
fn the_places_are_tried_in_order() {
    let own = Path::new("/usr/bin");
    let got = resources(None, Some(own), |p| p == Path::new("/usr/share/miyu"));
    assert_eq!(got, Some(PathBuf::from("/usr/share/miyu")));
    let app = Path::new("/A/Miyu.app/Contents/MacOS");
    let got = resources(None, Some(app), |p| p.ends_with("Resources/miyu"));
    assert_eq!(
        got,
        Some(PathBuf::from("/A/Miyu.app/Contents/Resources/miyu"))
    );
    let win = Path::new("/P/Miyu");
    let got = resources(None, Some(win), |_| true);
    assert_eq!(got, Some(PathBuf::from("/P/Miyu/resources")));
    assert_eq!(resources(None, Some(win), |_| false), None);
}

#[test]
fn the_plain_page_carries_the_escaped_words() {
    let html = plain("<b>&\"</b>");
    assert!(
        html.contains("<p>&lt;b&gt;&amp;&quot;&lt;/b&gt;</p>"),
        "{html}"
    );
}

#[test]
fn the_page_is_a_base64_data_url() {
    let url = page("正在启动 Miyu…");
    assert!(url.starts_with("data:text/html;charset=utf-8;base64,"));
    let parsed: url::Url = url.parse().unwrap();
    assert_eq!(parsed.scheme(), "data");
}
