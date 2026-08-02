#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct User {
    pub login: String,
    pub name: String,
    pub bio: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub avatar_url: String,
    pub html_url: String,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}

#[derive(Debug, Clone, Default)]
pub struct CheckResult {
    pub id: String,
    pub label: String,
    pub passed: bool,
    pub weight: u32,
    pub detail: String,
}

#[derive(Debug, Clone, Default)]
pub struct ReadmeScore {
    pub total: u32,
    pub max: u32,
    pub checks: Vec<CheckResult>,
    pub grade: String,
    pub summary: String,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Repo {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub html_url: String,
    pub language: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub fork: bool,
    pub archived: bool,
    pub topics: Vec<String>,
    pub default_branch: String,
    pub readme: String,
    pub score: ReadmeScore,
}

#[derive(Debug, Clone, Default)]
pub struct Portfolio {
    pub user: User,
    pub repos: Vec<Repo>,
    pub languages: Vec<(String, u32)>,
    pub average_score: f64,
    pub generated_at_utc: String,
}
