use super::*;

fn dir() -> PathBuf {
    PathBuf::from("down")
}

fn nothing_there(_: &Path) -> bool {
    false
}

#[test]
fn a_free_name_is_used_as_it_is() {
    let got = destination(&dir(), Some(OsStr::new("note.txt")), nothing_there);
    assert_eq!(got, dir().join("note.txt"));
}

#[test]
fn taken_names_get_a_number() {
    let taken = [dir().join("note.txt"), dir().join("note (1).txt")];
    let got = destination(&dir(), Some(OsStr::new("note.txt")), |p| {
        taken.iter().any(|t| t == p)
    });
    assert_eq!(got, dir().join("note (2).txt"));
}

#[test]
fn a_name_without_extension_gets_a_number_too() {
    let taken = dir().join("README");
    let got = destination(&dir(), Some(OsStr::new("README")), |p| p == taken);
    assert_eq!(got, dir().join("README (1)"));
}

#[test]
fn no_name_is_called_download() {
    assert_eq!(
        destination(&dir(), None, nothing_there),
        dir().join("download")
    );
    assert_eq!(
        destination(&dir(), Some(OsStr::new("")), nothing_there),
        dir().join("download")
    );
}

#[test]
fn only_the_last_part_of_a_path_is_kept() {
    let got = destination(
        &dir(),
        Some(OsStr::new("/home/x/Downloads/pic.png")),
        nothing_there,
    );
    assert_eq!(got, dir().join("pic.png"));
    let got = destination(&dir(), Some(OsStr::new("../../etc/passwd")), nothing_there);
    assert_eq!(got, dir().join("passwd"));
}

#[test]
fn dot_names_are_not_used() {
    assert_eq!(
        destination(&dir(), Some(OsStr::new("..")), nothing_there),
        dir().join("download")
    );
    assert_eq!(
        destination(&dir(), Some(OsStr::new(".")), nothing_there),
        dir().join("download")
    );
}
