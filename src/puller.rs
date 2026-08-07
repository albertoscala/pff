use std::{env, fs};
use serde::{Deserialize, Serialize};
use std::io::{self, IsTerminal};

static GITHUB: &str = "https://www.github.com";
static RAW_GITHUB: &str = "https://raw.githubusercontent.com";
static MIDDLE: &str = "main";
static PULLER: &str = "puller.toml";
static PULLED: &str = "pulled.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    repos: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Puller {
    dependencies: Dependencies,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setup {
    files: Vec<(String, String)>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pulled {
    setup: Setup,
}

impl Puller {
    pub fn fetch() -> Self{
        // Find the puller file
        let mut puller_file = env::current_dir().unwrap();
        puller_file.push(PULLER);

        // Parse the puller file
        let content = fs::read_to_string(puller_file).unwrap();
        let puller: Puller = toml::from_str(&content).unwrap();
        
        puller
    }

    pub fn display(&self) {
        let n = self.dependencies.repos.len();
        if n == 0 {
            status("Resolving", "no dependencies");
            return;
        }

        let color = io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none();
        let (bold, dim, reset) = if color {
            ("\x1b[1m", "\x1b[2m", "\x1b[0m")
        } else {
            ("", "", "")
        };

        eprintln!(
            "{bold}Project dependencies{reset} {dim}({n} {}){reset}",
            if n == 1 { "package" } else { "packages" }
        );

        let mut it = self.dependencies.repos.iter().peekable();
        while let Some((owner, repo)) = it.next() {
            let last = it.peek().is_none();
            let (branch, cont) = if last { ("└─", "  ") } else { ("├─", "│ ") };
            eprintln!("{dim}{branch}{reset} {bold}{repo}{reset} {dim}by {owner}{reset}");
            eprintln!("{dim}{cont}  {GITHUB}/{owner}/{repo}{reset}");
        }
        eprintln!();
    }

    pub fn pull(&self) {
        for (owner, repo) in &self.dependencies.repos {
            // Dialog
            status("Resolving", &format!("{owner}/{repo}"));

            // Compose url for pulled_file
            let url = format!("{}/{}/{}/{}/{}", RAW_GITHUB, owner, repo, MIDDLE, PULLED);
            
            // Parse the pulled file
            let response = reqwest::blocking::get(url).unwrap();
            let pulled: Pulled = toml::from_str(&response.text().unwrap()).unwrap();

            for (file, dir) in &pulled.setup.files {
                // Dialog
                status("Downloading", file);
                
                // Compose url for component to pull
                let url = format!("{}/{}/{}/{}/{}", RAW_GITHUB, owner, repo, MIDDLE, file);
                // File content
                let content = reqwest::blocking::get(url).unwrap()
                                                        .text().unwrap();
                // Write the file in the right dir
                let destination_dir = format!("{}{}", dir, file);
                match fs::write(&destination_dir, content) {
                    // Better dialogs
                    Ok(())  => status("Wrote", &destination_dir),
                    Err(e)  => status_err("Failed", &format!("{destination_dir}: {e}")),
                }
            }
        }
    }
}

// Helpers


fn status(verb: &str, msg: &str) {
    if io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        eprintln!("\x1b[1;32m{verb:>12}\x1b[0m {msg}");
    } else {
        eprintln!("{verb:>12} {msg}");
    }
}

fn status_err(verb: &str, msg: &str) {
    if io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        eprintln!("\x1b[1;31m{verb:>12}\x1b[0m {msg}");
    } else {
        eprintln!("{verb:>12} {msg}");
    }
}