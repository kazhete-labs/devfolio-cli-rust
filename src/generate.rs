use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::emit;
use crate::github::GitHubClient;
use crate::model::Portfolio;
use crate::score;

pub struct Options {
    pub login: String,
    pub out_dir: PathBuf,
    pub max_repos: usize,
    pub skip_readme: bool,
}

pub struct ResultOut {
    pub portfolio: Portfolio,
    pub out_dir: PathBuf,
}

pub struct Runner {
    pub gh: GitHubClient,
}

impl Runner {
    pub fn new(token: Option<String>) -> Result<Self> {
        Ok(Self {
            gh: GitHubClient::new(token)?,
        })
    }

    pub fn with_client(gh: GitHubClient) -> Self {
        Self { gh }
    }

    pub fn run(&self, mut opt: Options) -> Result<ResultOut> {
        if opt.login.is_empty() {
            bail!("github username is required");
        }
        if opt.out_dir.as_os_str().is_empty() {
            opt.out_dir = PathBuf::from("devfolio-out");
        }
        if opt.max_repos == 0 {
            opt.max_repos = 12;
        }

        let user = self
            .gh
            .fetch_user(&opt.login)
            .context("fetch user")?;
        let mut repos = self
            .gh
            .fetch_repos(&opt.login)
            .context("fetch repos")?;

        repos.sort_by(|a, b| {
            b.stargazers_count
                .cmp(&a.stargazers_count)
                .then_with(|| a.name.cmp(&b.name))
        });
        if repos.len() > opt.max_repos {
            repos.truncate(opt.max_repos);
        }

        let mut langs: HashMap<String, u32> = HashMap::new();
        let mut sum = 0.0;
        let mut scored = 0u32;
        for repo in &mut repos {
            if !repo.language.is_empty() {
                *langs.entry(repo.language.clone()).or_default() += 1;
            }
            if !opt.skip_readme {
                repo.readme = self
                    .gh
                    .fetch_readme(&opt.login, &repo.name)
                    .with_context(|| format!("readme {}", repo.name))?;
            }
            repo.score = score::score_readme(&repo.readme);
            if repo.score.max > 0 {
                sum += f64::from(repo.score.total) / f64::from(repo.score.max) * 100.0;
                scored += 1;
            }
        }

        let avg = if scored > 0 {
            sum / f64::from(scored)
        } else {
            0.0
        };

        let mut lang_vec: Vec<(String, u32)> = langs.into_iter().collect();
        lang_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // RFC3339-ish UTC without chrono dep
        let generated = format!("{now}");

        let portfolio = Portfolio {
            user,
            repos,
            languages: lang_vec,
            average_score: avg,
            generated_at_utc: generated,
        };
        emit::write_portfolio(Path::new(&opt.out_dir), &portfolio)?;
        Ok(ResultOut {
            portfolio,
            out_dir: opt.out_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn end_to_end_with_mock_api() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/octocat");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"login":"octocat","name":"The Octocat","bio":"hi","avatar_url":"https://x/a.png","html_url":"https://github.com/octocat","public_repos":1,"followers":1,"following":0}"#);
        });
        server.mock(|when, then| {
            when.method(GET).path("/users/octocat/repos");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"[{"name":"Hello-World","full_name":"octocat/Hello-World","description":"demo","html_url":"https://github.com/octocat/Hello-World","language":"Rust","stargazers_count":5,"fork":false}]"#);
        });
        server.mock(|when, then| {
            when.method(GET).path("/repos/octocat/Hello-World/readme");
            then.status(200).body("# Hi\n\n## Install\n\ncargo install x\n");
        });

        let dir = tempfile::tempdir().unwrap();
        let runner = Runner::with_client(
            GitHubClient::new(None)
                .unwrap()
                .with_base_url(server.base_url()),
        );
        let out = runner
            .run(Options {
                login: "octocat".into(),
                out_dir: dir.path().to_path_buf(),
                max_repos: 5,
                skip_readme: false,
            })
            .unwrap();
        assert_eq!(out.portfolio.user.login, "octocat");
        assert_eq!(out.portfolio.repos.len(), 1);
        assert!(dir.path().join("index.html").exists());
        assert!(dir.path().join("scorecard.md").exists());
    }
}
