use std::time::Instant;

fn main() {
    let mut md = String::from(
        "# Title\n\n## Install\n\n```\ncargo install x\n```\n\n## Demo\n\n![d](d.gif)\n\n## License\n\nMIT License\n\n## Architecture\n\nflow\n![ci](https://img.shields.io/badge/x-y-blue)\n",
    );
    while md.len() < 500 {
        md.push_str(" word");
    }
    let n = 20_000usize;
    let start = Instant::now();
    for _ in 0..n {
        let _ = devfolio_cli_rust::score::score_readme(&md);
    }
    println!("{:.6}", start.elapsed().as_secs_f64());
}
