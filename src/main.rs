use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use devfolio_cli_rust::generate;

#[derive(Parser, Debug)]
#[command(
    name = "devfolio",
    about = "Turn a GitHub username into a portfolio site + README scorecard",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetch profile, score READMEs, emit static portfolio
    Generate {
        /// GitHub username
        user: String,
        /// Output directory
        #[arg(short, long, default_value = "devfolio-out")]
        out: PathBuf,
        /// Max non-fork repos to include
        #[arg(long, default_value_t = 12)]
        max_repos: usize,
        /// GitHub token (or set GITHUB_TOKEN)
        #[arg(long)]
        token: Option<String>,
        /// Overall timeout seconds
        #[arg(long, default_value_t = 120)]
        timeout: u64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate {
            user,
            out,
            max_repos,
            token,
            timeout: _,
        } => {
            let runner = generate::Runner::new(token)?;
            let res = runner.run(generate::Options {
                login: user,
                out_dir: out,
                max_repos,
                skip_readme: false,
            })?;
            println!("Generated portfolio for @{}", res.portfolio.user.login);
            println!("  repos scored: {}", res.portfolio.repos.len());
            println!("  avg README:   {:.1} / 100", res.portfolio.average_score);
            println!("  output:       {}", res.out_dir.display());
            println!("  open:         {}/index.html", res.out_dir.display());
            println!("  scorecard:    {}/scorecard.md", res.out_dir.display());
        }
    }
    Ok(())
}
