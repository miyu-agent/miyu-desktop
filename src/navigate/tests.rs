use super::*;

fn url(s: &str) -> Url {
    s.parse().unwrap()
}

fn origin() -> Option<Origin> {
    Origin::of(&url("http://127.0.0.1:8300/#ticket=abc"))
}

#[test]
fn the_web_apps_own_pages_stay_in_the_window() {
    assert_eq!(
        navigation(origin(), &url("http://127.0.0.1:8300/")),
        Go::Stay
    );
    assert_eq!(
        navigation(origin(), &url("http://127.0.0.1:8300/p/t/miyu-x/")),
        Go::Stay
    );
}

#[test]
fn another_port_or_host_is_not_the_same_origin() {
    assert_eq!(
        navigation(origin(), &url("http://127.0.0.1:8301/")),
        Go::Elsewhere
    );
    assert_eq!(
        navigation(origin(), &url("http://localhost:8300/")),
        Go::Elsewhere
    );
    assert_eq!(
        navigation(origin(), &url("https://127.0.0.1:8300/")),
        Go::Elsewhere
    );
}

#[test]
fn before_the_address_is_known_only_blank_pages_stay() {
    assert_eq!(navigation(None, &url("about:blank")), Go::Stay);
    assert_eq!(
        navigation(None, &url("http://127.0.0.1:8300/")),
        Go::Elsewhere
    );
}

#[test]
fn pages_the_web_view_makes_itself_stay() {
    assert_eq!(
        navigation(origin(), &url("blob:http://127.0.0.1:8300/x")),
        Go::Stay
    );
    assert_eq!(navigation(origin(), &url("data:text/plain,hi")), Go::Stay);
}

#[test]
fn links_out_go_to_the_system_browser() {
    assert_eq!(
        navigation(origin(), &url("https://example.com/a")),
        Go::Elsewhere
    );
    assert_eq!(
        navigation(origin(), &url("mailto:a@example.com")),
        Go::Elsewhere
    );
}

#[test]
fn other_schemes_are_not_opened() {
    assert_eq!(navigation(origin(), &url("file:///etc/passwd")), Go::Drop);
    assert_eq!(navigation(origin(), &url("javascript:alert(1)")), Go::Drop);
    assert_eq!(navigation(origin(), &url("miyu://x")), Go::Drop);
}

#[test]
fn new_windows_always_leave_even_for_the_same_origin() {
    assert_eq!(new_window(&url("http://127.0.0.1:8300/")), Go::Elsewhere);
    assert_eq!(new_window(&url("https://example.com/")), Go::Elsewhere);
    assert_eq!(new_window(&url("file:///tmp/x")), Go::Drop);
}

#[test]
fn only_a_loopback_http_address_with_a_port_is_an_origin() {
    assert!(Origin::of(&url("http://127.0.0.1/")).is_none());
    assert!(Origin::of(&url("http://192.168.1.2:8300/")).is_none());
    assert!(Origin::of(&url("https://127.0.0.1:8300/")).is_none());
}
