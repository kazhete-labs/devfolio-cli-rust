use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::model::Portfolio;

const STYLES: &str = r#":root {
  --bg: #f4f1ea;
  --ink: #1c1917;
  --muted: #57534e;
  --card: #fffdf8;
  --line: #d6d3d1;
  --accent: #0f766e;
  --font: "Iowan Old Style", "Palatino Linotype", Palatino, Georgia, serif;
  --sans: "Segoe UI", system-ui, sans-serif;
}
* { box-sizing: border-box; }
body {
  margin: 0;
  color: var(--ink);
  background:
    radial-gradient(circle at 10% 10%, #fde68a55, transparent 40%),
    radial-gradient(circle at 90% 0%, #99f6e455, transparent 35%),
    var(--bg);
  font-family: var(--sans);
  line-height: 1.5;
}
.hero, main { max-width: 960px; margin: 0 auto; padding: 2rem 1.25rem; }
.hero { display: flex; gap: 1.25rem; align-items: center; }
.avatar { width: 96px; height: 96px; border-radius: 20px; object-fit: cover; border: 1px solid var(--line); }
h1, h2, h3 { font-family: var(--font); letter-spacing: -0.02em; }
.eyebrow { text-transform: uppercase; letter-spacing: 0.12em; font-size: 0.75rem; color: var(--accent); margin: 0 0 0.25rem; }
.bio { color: var(--muted); max-width: 42rem; }
.meta, .muted, .footer { color: var(--muted); font-size: 0.95rem; }
.grid { display: grid; gap: 1rem; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); }
.card { background: var(--card); border: 1px solid var(--line); border-radius: 14px; padding: 1rem 1.1rem; }
.card a { color: var(--ink); text-decoration: none; }
.card a:hover { color: var(--accent); }
.langs { list-style: none; padding: 0; display: grid; gap: 0.35rem; max-width: 320px; }
.langs li { display: flex; justify-content: space-between; border-bottom: 1px dashed var(--line); padding: 0.25rem 0; }
a { color: var(--accent); }
"#;

pub fn write_portfolio(out_dir: &Path, p: &Portfolio) -> Result<()> {
    fs::create_dir_all(out_dir).with_context(|| format!("mkdir {}", out_dir.display()))?;
    fs::write(out_dir.join("styles.css"), STYLES)?;
    fs::write(out_dir.join("index.html"), render_index(p))?;
    fs::write(out_dir.join("scorecard.html"), render_scorecard_html(p))?;
    fs::write(out_dir.join("scorecard.md"), render_scorecard_md(p))?;
    Ok(())
}

fn e(s: &str) -> String {
    html_escape::encode_text(s).into_owned()
}

fn render_index(p: &Portfolio) -> String {
    let name = if p.user.name.is_empty() {
        &p.user.login
    } else {
        &p.user.name
    };
    let mut repos = String::new();
    for r in &p.repos {
        let lang = if r.language.is_empty() {
            "n/a"
        } else {
            &r.language
        };
        repos.push_str(&format!(
            r#"
      <article class="card">
        <h3><a href="{url}">{name}</a></h3>
        <p class="muted">{desc}</p>
        <p class="meta">★ {stars} · {lang} · README {grade} ({total}/{max})</p>
      </article>"#,
            url = e(&r.html_url),
            name = e(&r.name),
            desc = e(&r.description),
            stars = r.stargazers_count,
            lang = e(lang),
            grade = e(&r.score.grade),
            total = r.score.total,
            max = r.score.max,
        ));
    }
    let mut langs = String::new();
    if p.languages.is_empty() {
        langs.push_str("<li><span>n/a</span><span>0</span></li>");
    } else {
        for (k, v) in &p.languages {
            langs.push_str(&format!(
                "<li><span>{}</span><span>{}</span></li>",
                e(k),
                v
            ));
        }
    }
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>{title} · Devfolio</title>
  <link rel="stylesheet" href="styles.css"/>
</head>
<body>
  <header class="hero">
    <img class="avatar" src="{avatar}" alt="avatar"/>
    <div>
      <p class="eyebrow">devfolio-cli-rust</p>
      <h1>{title}</h1>
      <p class="bio">{bio}</p>
      <p class="meta"><a href="{html_url}">@{login}</a> · {repos_n} public repos · {followers} followers · avg README {avg:.0}/100</p>
    </div>
  </header>
  <main>
    <section>
      <h2>Languages</h2>
      <ul class="langs">{langs}</ul>
    </section>
    <section>
      <h2>Repositories</h2>
      <div class="grid">{repo_cards}</div>
    </section>
    <p class="footer"><a href="scorecard.html">Open README scorecard</a> · generated {when} UTC</p>
  </main>
</body>
</html>
"#,
        title = e(name),
        avatar = e(&p.user.avatar_url),
        bio = e(&p.user.bio),
        html_url = e(&p.user.html_url),
        login = e(&p.user.login),
        repos_n = p.user.public_repos,
        followers = p.user.followers,
        avg = p.average_score,
        langs = langs,
        repo_cards = repos,
        when = e(&p.generated_at_utc),
    )
}

fn render_scorecard_html(p: &Portfolio) -> String {
    let mut body = String::new();
    for r in &p.repos {
        body.push_str(&format!(
            "<h3>{} — {} ({}/{})</h3><ul>",
            e(&r.name),
            e(&r.score.grade),
            r.score.total,
            r.score.max
        ));
        for c in &r.score.checks {
            let mark = if c.passed { "PASS" } else { "FAIL" };
            body.push_str(&format!(
                "<li><strong>{}</strong> [{}] {} — {}</li>",
                mark,
                e(&c.id),
                e(&c.label),
                e(&c.detail)
            ));
        }
        body.push_str("</ul>");
    }
    format!(
        r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8"/><title>Scorecard · {login}</title><link rel="stylesheet" href="styles.css"/></head>
<body><main><p><a href="index.html">← Portfolio</a></p><h1>README scorecard</h1><p class="muted">@{login} · avg {avg:.0}/100</p>{body}</main></body></html>"#,
        login = e(&p.user.login),
        avg = p.average_score,
        body = body
    )
}

fn render_scorecard_md(p: &Portfolio) -> String {
    let mut b = format!(
        "# README scorecard — @{}\n\nAverage score: **{:.1} / 100** · generated {} UTC\n\n",
        p.user.login, p.average_score, p.generated_at_utc
    );
    for r in &p.repos {
        b.push_str(&format!(
            "## {} — grade {} ({}/{})\n\n{}\n\n| Check | Result | Weight | Detail |\n|---|---|---:|---|\n",
            r.name, r.score.grade, r.score.total, r.score.max, r.score.summary
        ));
        for c in &r.score.checks {
            let res = if c.passed { "PASS" } else { "FAIL" };
            b.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                c.label, res, c.weight, c.detail
            ));
        }
        b.push('\n');
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CheckResult, ReadmeScore, Repo, User};

    #[test]
    fn writes_all_artifacts() {
        let dir = tempfile::tempdir().unwrap();
        let p = Portfolio {
            user: User {
                login: "octocat".into(),
                name: "The Octocat".into(),
                bio: "GitHub mascot".into(),
                avatar_url: "https://example.com/a.png".into(),
                html_url: "https://github.com/octocat".into(),
                ..Default::default()
            },
            repos: vec![Repo {
                name: "Hello-World".into(),
                description: "demo".into(),
                html_url: "https://github.com/octocat/Hello-World".into(),
                language: "Rust".into(),
                stargazers_count: 3,
                score: ReadmeScore {
                    total: 50,
                    max: 100,
                    grade: "D".into(),
                    summary: "Improve: install".into(),
                    checks: vec![CheckResult {
                        id: "install".into(),
                        label: "Install".into(),
                        passed: false,
                        weight: 20,
                        detail: "missing".into(),
                    }],
                },
                ..Default::default()
            }],
            languages: vec![("Rust".into(), 1)],
            average_score: 50.0,
            generated_at_utc: "2026-08-02T00:00:00Z".into(),
        };
        write_portfolio(dir.path(), &p).unwrap();
        for name in ["index.html", "scorecard.html", "scorecard.md", "styles.css"] {
            assert!(dir.path().join(name).exists(), "missing {name}");
        }
        let md = fs::read_to_string(dir.path().join("scorecard.md")).unwrap();
        assert!(md.len() > 20);
    }
}
