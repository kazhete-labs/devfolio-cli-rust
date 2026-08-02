use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::env;
use std::time::Duration;

use crate::model::{Repo, User};

#[derive(Debug, Deserialize)]
struct ApiUser {
    login: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    bio: Option<String>,
    #[serde(default)]
    company: Option<String>,
    #[serde(default)]
    blog: Option<String>,
    #[serde(default)]
    location: Option<String>,
    #[serde(default)]
    avatar_url: Option<String>,
    #[serde(default)]
    html_url: Option<String>,
    #[serde(default)]
    public_repos: u32,
    #[serde(default)]
    followers: u32,
    #[serde(default)]
    following: u32,
}

#[derive(Debug, Deserialize)]
struct ApiRepo {
    name: String,
    full_name: String,
    #[serde(default)]
    description: Option<String>,
    html_url: String,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    stargazers_count: u32,
    #[serde(default)]
    forks_count: u32,
    #[serde(default)]
    fork: bool,
    #[serde(default)]
    archived: bool,
    #[serde(default)]
    topics: Vec<String>,
    #[serde(default)]
    default_branch: Option<String>,
}

pub struct GitHubClient {
    http: Client,
    base_url: String,
    token: Option<String>,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Result<Self> {
        let token = token.or_else(|| env::var("GITHUB_TOKEN").ok());
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        Ok(Self {
            http,
            base_url: "https://api.github.com".into(),
            token,
        })
    }

    pub fn with_base_url(mut self, base: impl Into<String>) -> Self {
        self.base_url = base.into();
        self
    }

    fn request(&self, path: &str, accept: &str) -> Result<reqwest::blocking::Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self
            .http
            .get(&url)
            .header(USER_AGENT, "devfolio-cli-rust")
            .header(ACCEPT, accept)
            .header("X-GitHub-Api-Version", "2022-11-28");
        if let Some(t) = &self.token {
            req = req.header(AUTHORIZATION, format!("Bearer {t}"));
        }
        let res = req.send().with_context(|| format!("GET {url}"))?;
        Ok(res)
    }

    pub fn fetch_user(&self, login: &str) -> Result<User> {
        let res = self.request(&format!("/users/{login}"), "application/vnd.github+json")?;
        if !res.status().is_success() {
            return Err(anyhow!("github user {}: {}", login, res.status()));
        }
        let u: ApiUser = res.json()?;
        Ok(User {
            login: u.login,
            name: u.name.unwrap_or_default(),
            bio: u.bio.unwrap_or_default(),
            company: u.company.unwrap_or_default(),
            blog: u.blog.unwrap_or_default(),
            location: u.location.unwrap_or_default(),
            avatar_url: u.avatar_url.unwrap_or_default(),
            html_url: u.html_url.unwrap_or_default(),
            public_repos: u.public_repos,
            followers: u.followers,
            following: u.following,
        })
    }

    pub fn fetch_repos(&self, login: &str) -> Result<Vec<Repo>> {
        let path = format!("/users/{login}/repos?per_page=100&sort=updated&type=owner");
        let res = self.request(&path, "application/vnd.github+json")?;
        if !res.status().is_success() {
            return Err(anyhow!("github repos {}: {}", login, res.status()));
        }
        let raw: Vec<ApiRepo> = res.json()?;
        Ok(raw
            .into_iter()
            .filter(|r| !r.fork)
            .map(|r| Repo {
                name: r.name,
                full_name: r.full_name,
                description: r.description.unwrap_or_default(),
                html_url: r.html_url,
                language: r.language.unwrap_or_default(),
                stargazers_count: r.stargazers_count,
                forks_count: r.forks_count,
                fork: r.fork,
                archived: r.archived,
                topics: r.topics,
                default_branch: r.default_branch.unwrap_or_else(|| "main".into()),
                ..Default::default()
            })
            .collect())
    }

    pub fn fetch_readme(&self, owner: &str, repo: &str) -> Result<String> {
        let res = self.request(
            &format!("/repos/{owner}/{repo}/readme"),
            "application/vnd.github.raw",
        )?;
        if res.status().as_u16() == 404 {
            return Ok(String::new());
        }
        if !res.status().is_success() {
            return Err(anyhow!(
                "github README {}/{}: {}",
                owner,
                repo,
                res.status()
            ));
        }
        Ok(res.text()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn fetch_user_repos_readme() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/octocat");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"login":"octocat","name":"The Octocat","bio":"hi","avatar_url":"https://x/a.png","html_url":"https://github.com/octocat","public_repos":2,"followers":1,"following":1}"#);
        });
        server.mock(|when, then| {
            when.method(GET).path("/users/octocat/repos");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"[{"name":"Hello-World","full_name":"octocat/Hello-World","description":"demo","html_url":"https://github.com/octocat/Hello-World","language":"Rust","stargazers_count":10,"forks_count":1,"fork":false,"archived":false,"topics":["demo"],"default_branch":"main"},{"name":"forked","full_name":"octocat/forked","html_url":"https://github.com/octocat/forked","fork":true}]"#);
        });
        server.mock(|when, then| {
            when.method(GET).path("/repos/octocat/Hello-World/readme");
            then.status(200)
                .body("# Hello\n\n## Install\n\ncargo install example\n");
        });

        let client = GitHubClient::new(None)
            .unwrap()
            .with_base_url(server.base_url());
        let u = client.fetch_user("octocat").unwrap();
        assert_eq!(u.login, "octocat");
        let repos = client.fetch_repos("octocat").unwrap();
        assert_eq!(repos.len(), 1);
        let md = client.fetch_readme("octocat", "Hello-World").unwrap();
        assert!(!md.is_empty());
    }
}
