use docusaurus_magic_comments::common::select_lines;

const CONTENT: &str = "line1\nline2\nline3\nline4\nline5\nline6\nline7\n";

#[test]
fn _0001() {
    assert_eq!(
        vec!["line1".to_string(), "line2".to_string()],
        select_lines(CONTENT, 0, 1)
    )
}

#[test]
fn _0002() {
    assert_eq!(
        vec![
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
            "line5".to_string(),
            "line6".to_string()
        ],
        select_lines(CONTENT, 1, 5)
    )
}

#[test]
fn _0003() {
    assert_eq!(
        vec![
            "line5".to_string(),
            "line6".to_string(),
            "line7".to_string()
        ],
        select_lines(CONTENT, 4, 100)
    )
}
