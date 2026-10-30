use super::*;

#[test]
fn no_debug_port_unless_asked() {
    assert_eq!(browser_args(None), None);
    assert_eq!(browser_args(Some("")), None);
    assert_eq!(browser_args(Some("abc")), None);
    assert_eq!(browser_args(Some("80")), None);
}

#[test]
fn a_debug_port_keeps_the_default_arguments() {
    let args = browser_args(Some(" 9222 ")).unwrap();
    assert!(args.contains("--remote-debugging-port=9222"));
    assert!(args.contains("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection"));
}
