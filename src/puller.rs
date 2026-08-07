use std::{env, fs};
use serde::{Deserialize, Serialize};

static GITHUB: &str = "https://raw.githubusercontent.com";
static MIDDLE: &str = "main";
static PULLER: &str = "puller.toml";
static PULLED: &str = "pulled.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    repos: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Puller {
    config: Config,
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

    pub fn pull(&self) {
        for entry in &self.dependencies.repos {
            // Compose url for pulled_file
            let url = format!("{}/{}/{}/{}/{}", GITHUB, self.config.username, entry, MIDDLE, PULLED);
            
            // Parse the pulled file
            let response = reqwest::blocking::get(url).unwrap();
            let pulled: Pulled = toml::from_str(&response.text().unwrap()).unwrap();

            for (file, dir) in &pulled.setup.files {
                // Compose url for component to pull
                let url = format!("{}/{}/{}/{}/{}", GITHUB, self.config.username, entry, MIDDLE, file);
                // File content
                let content = reqwest::blocking::get(url).unwrap()
                                                        .text().unwrap();
                // Write the file in the right dir
                let destination_dir = format!("{}{}", dir, file);
                match fs::write(&destination_dir, content) {
                    Ok(()) => println!("Wrote {} successfully", &destination_dir),
                    Err(e) => eprintln!("Failed to write {}: {}", &destination_dir, e),
                }
            }
        }
    }
}