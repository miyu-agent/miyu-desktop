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

#[test]
fn the_first_window_is_three_quarters_of_a_big_screen() {
    assert_eq!(first_size(2560.0, 1440.0), (1920.0, 1080.0));
}

#[test]
fn the_first_window_is_not_smaller_than_the_least() {
    assert_eq!(first_size(1440.0, 900.0), (1100.0, 760.0));
}

#[test]
fn the_first_window_fits_a_small_screen() {
    assert_eq!(first_size(1024.0, 768.0), (922.0, 691.0));
}
