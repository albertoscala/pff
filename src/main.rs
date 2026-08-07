mod puller;

use std::io::Write;

use crate::puller::Puller;

fn main() {
    // Retrive puller file
    let puller = Puller::fetch();

    // Print the dependencies presents in the project
    puller.display();

    // Ask the user to proceed or abort
    print!("\x1b[1;36m::\x1b[0m Proceed with resolution? [y/N] ");
    let _ = std::io::stdout().flush();

    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();

    match line.trim() {
        "y" => {
            // Pull the entrier
            puller.pull();
        },
        _ => {
            eprintln!("\x1b[1;33mnote\x1b[0m: aborted at user request; nothing was written");
            return;
        }
    }
}
