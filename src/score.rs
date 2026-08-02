use std::sync::LazyLock;

use regex::Regex;

use crate::model::{CheckResult, ReadmeScore};

static RE_HEADING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^#{1,3}\s+\S+").unwrap());
static RE_BADGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)!\[.*?\]\(https?://.*(shields\.io|badge|img\.shields\.io).*\)|\[!\[.*?\]\(https?://.*\)\]\(https?://.*\)").unwrap()
});
static RE_INSTALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?im)^#{1,3}\s+.*(install|getting started|quick ?start|usage).*$|go install |npm (i|install) |pip install |cargo install |docker (compose )?up").unwrap()
});
static RE_DEMO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?im)^#{1,3}\s+.*(demo|screenshot|preview).*$|\.(gif|webm|mp4)\)|ASCIinema|loom\.com").unwrap()
});
static RE_LICENSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?im)^#{1,3}\s+.*license.*$|mit license|apache license|spdx-license").unwrap()
});
static RE_ARCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?im)^#{1,3}\s+.*(architecture|design|how it works|overview).*$").unwrap()
});
static RE_CODE_FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^```").unwrap());

fn check(id: &str, label: &str, weight: u32, passed: bool, detail: &str) -> CheckResult {
    CheckResult {
        id: id.to_string(),
        label: label.to_string(),
        passed,
        weight,
        detail: if passed {
            "ok".to_string()
        } else {
            detail.to_string()
        },
    }
}

fn grade(total: u32, max: u32) -> String {
    if max == 0 {
        return "F".into();
    }
    let pct = f64::from(total) / f64::from(max) * 100.0;
    match pct {
        p if p >= 90.0 => "A",
        p if p >= 80.0 => "B",
        p if p >= 70.0 => "C",
        p if p >= 55.0 => "D",
        _ => "F",
    }
    .into()
}

fn summarize(checks: &[CheckResult]) -> String {
    let missing: Vec<&str> = checks
        .iter()
        .filter(|c| !c.passed)
        .map(|c| c.id.as_str())
        .collect();
    if missing.is_empty() {
        "Excellent README coverage.".into()
    } else {
        format!("Improve: {}", missing.join(", "))
    }
}

/// Score README markdown quality (max 100).
pub fn score_readme(md: &str) -> ReadmeScore {
    let checks = vec![
        check(
            "non_empty",
            "README is non-empty",
            10,
            !md.trim().is_empty(),
            "file missing or empty",
        ),
        check(
            "title",
            "Has a markdown heading",
            10,
            RE_HEADING.is_match(md),
            "add an H1/H2 title",
        ),
        check(
            "length",
            "Adequate length (≥400 chars)",
            10,
            md.trim().chars().count() >= 400,
            "expand the README",
        ),
        check(
            "install",
            "Install / getting started section",
            20,
            RE_INSTALL.is_match(md),
            "document how to install/run",
        ),
        check(
            "demo",
            "Demo / screenshot evidence",
            15,
            RE_DEMO.is_match(md),
            "add a GIF, screenshot, or demo link",
        ),
        check(
            "badges",
            "Status badges",
            10,
            RE_BADGE.is_match(md),
            "add shields.io (or similar) badges",
        ),
        check(
            "license",
            "License mentioned",
            10,
            RE_LICENSE.is_match(md),
            "add a License section",
        ),
        check(
            "architecture",
            "Architecture / how-it-works",
            10,
            RE_ARCH.is_match(md),
            "sketch architecture briefly",
        ),
        check(
            "code_samples",
            "Code fences / examples",
            5,
            RE_CODE_FENCE.is_match(md),
            "include copy-pasteable examples",
        ),
    ];

    let mut total = 0u32;
    let mut max = 0u32;
    for c in &checks {
        max += c.weight;
        if c.passed {
            total += c.weight;
        }
    }

    ReadmeScore {
        grade: grade(total, max),
        summary: summarize(&checks),
        total,
        max,
        checks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excellent_readme_scores_high() {
        let md = r#"# Devfolio

![ci](https://img.shields.io/badge/ci-passing-brightgreen)

## Install

```
cargo install --path .
```

## Demo

![demo](docs/demo.gif)

## Architecture

CLI → GitHub adapter → scoring domain → static emit.

## License

MIT License
"#;
        let s = score_readme(md);
        assert!(s.total >= 80, "got {}/{} {}", s.total, s.max, s.summary);
        assert!(s.grade == "A" || s.grade == "B");
    }

    #[test]
    fn empty_readme_is_f() {
        let s = score_readme("");
        assert_eq!(s.total, 0);
        assert_eq!(s.grade, "F");
    }

    #[test]
    fn partial_fails_install() {
        let s = score_readme("# Tiny\n\nShort readme without install demo or license.");
        assert!(s.checks.iter().any(|c| c.id == "non_empty" && c.passed));
        assert!(s.checks.iter().any(|c| c.id == "install" && !c.passed));
    }

    #[test]
    fn table_driven_cases() {
        struct Case {
            name: &'static str,
            md: &'static str,
            want_min: u32,
            want_max: u32,
            must_pass: &'static [&'static str],
            must_fail: &'static [&'static str],
        }
        let cases = [
            Case {
                name: "empty",
                md: "",
                want_min: 0,
                want_max: 0,
                must_pass: &[],
                must_fail: &["non_empty", "install"],
            },
            Case {
                name: "title_only",
                md: "# Title\n",
                want_min: 10,
                want_max: 30,
                must_pass: &["non_empty", "title"],
                must_fail: &["install", "demo"],
            },
        ];
        for tc in cases {
            let s = score_readme(tc.md);
            assert!(
                s.total >= tc.want_min && s.total <= tc.want_max,
                "{}: score {} not in [{},{}]",
                tc.name,
                s.total,
                tc.want_min,
                tc.want_max
            );
            for id in tc.must_pass {
                assert!(
                    s.checks.iter().any(|c| c.id == *id && c.passed),
                    "{}: {} should pass",
                    tc.name,
                    id
                );
            }
            for id in tc.must_fail {
                assert!(
                    s.checks.iter().any(|c| c.id == *id && !c.passed),
                    "{}: {} should fail",
                    tc.name,
                    id
                );
            }
        }
    }
}
