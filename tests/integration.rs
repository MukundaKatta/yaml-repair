use yaml_repair::repair;

#[test]
fn strips_yaml_fence() {
    let raw = "Sure:\n```yaml\nname: Claude\n```\nDone!";
    let fixed = repair(raw);
    assert_eq!(fixed, "name: Claude");
}

#[test]
fn dedents_when_fence_content_is_indented() {
    let raw = "```yaml\n  name: Claude\n  tools:\n    - read\n```";
    let fixed = repair(raw);
    assert!(fixed.starts_with("name: Claude"));
    assert!(fixed.contains("tools:\n  - read"), "got:\n{fixed}");
}

#[test]
fn normalizes_crlf() {
    let raw = "a: 1\r\nb: 2\r\n";
    let fixed = repair(raw);
    assert!(!fixed.contains('\r'));
}

#[test]
fn converts_leading_tabs_to_spaces() {
    let raw = "a:\n\tb: 1\n";
    let fixed = repair(raw);
    assert!(!fixed.contains('\t'), "tabs not converted: {fixed:?}");
}

#[test]
fn trims_trailing_whitespace() {
    let raw = "a: 1   \nb: 2\t\n";
    let fixed = repair(raw);
    assert!(!fixed.contains("   \n"));
    assert!(!fixed.contains("\t\n"));
}

#[test]
fn no_fence_passes_through() {
    let raw = "a: 1\nb: 2";
    assert_eq!(repair(raw), "a: 1\nb: 2");
}
