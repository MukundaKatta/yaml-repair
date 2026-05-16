//! # yaml-repair
//!
//! Repair messy YAML emitted by LLMs into something a real YAML parser
//! will accept.
//!
//! Fixes applied:
//!
//! 1. Strip ```yaml / ``` fences and surrounding prose.
//! 2. Normalize CRLF and CR line endings to LF.
//! 3. Convert leading tabs to 2 spaces (YAML forbids leading tabs in
//!    indentation).
//! 4. Dedent so the shallowest non-empty line is at column 0.
//! 5. Trim trailing whitespace on each line.
//!
//! ## Example
//!
//! ```
//! use yaml_repair::repair;
//! let raw = "Sure:\n```yaml\n  name: Claude\n  tools:\n    - read\n    - write\n```";
//! let fixed = repair(raw);
//! assert!(fixed.starts_with("name: Claude"));
//! assert!(!fixed.contains("```"));
//! ```

#![deny(missing_docs)]

/// Clean `raw` and return YAML-parser-ready text.
pub fn repair(raw: &str) -> String {
    let mut s = strip_fences(raw);
    s = normalize_line_endings(&s);
    s = tabs_to_spaces(&s);
    s = trim_trailing_ws(&s);
    s = dedent(&s);
    while s.ends_with('\n') {
        s.pop();
    }
    s
}

fn strip_fences(s: &str) -> String {
    // Reuse the inner-block logic: find first ``` line, take until next ```.
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 2 < bytes.len() {
        if &bytes[i..i + 3] == b"```" {
            let mut start = i + 3;
            while start < bytes.len() && bytes[start] != b'\n' {
                start += 1;
            }
            if start >= bytes.len() {
                return s.to_string();
            }
            start += 1;
            let mut j = start;
            while j + 3 <= bytes.len() {
                if &bytes[j..j + 3] == b"```" {
                    let prev = j.checked_sub(1).map(|k| bytes[k]).unwrap_or(b'\n');
                    if prev == b'\n' {
                        return s[start..j].to_string();
                    }
                }
                j += 1;
            }
            return s.to_string();
        }
        i += 1;
    }
    s.to_string()
}

fn normalize_line_endings(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}

fn tabs_to_spaces(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for line in s.split_inclusive('\n') {
        // Convert tabs only in the leading indentation.
        let mut chars = line.chars().peekable();
        let mut in_indent = true;
        for c in chars.by_ref() {
            if in_indent && c == '\t' {
                out.push_str("  ");
            } else {
                if c != ' ' && c != '\t' && c != '\n' {
                    in_indent = false;
                }
                out.push(c);
            }
        }
    }
    out
}

fn trim_trailing_ws(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for line in s.split_inclusive('\n') {
        let had_nl = line.ends_with('\n');
        let core = if had_nl { &line[..line.len() - 1] } else { line };
        let stripped = core.trim_end_matches(|c: char| c == ' ' || c == '\t');
        out.push_str(stripped);
        if had_nl {
            out.push('\n');
        }
    }
    out
}

fn dedent(s: &str) -> String {
    let min_indent = s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().take_while(|c| *c == ' ').count())
        .min()
        .unwrap_or(0);
    if min_indent == 0 {
        return s.to_string();
    }
    s.lines()
        .map(|l| {
            if l.len() >= min_indent {
                &l[min_indent..]
            } else {
                l
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
