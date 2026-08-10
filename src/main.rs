mod puller;
mod dialog;

use std::io::Write;
use crate::{puller::Puller};
use crate::dialog::{prompt, note, status, error};

fn main() {
    // Retrive puller file
    let puller = Puller::fetch();

    // Print the dependencies presents in the project
    puller.display();

    // Ask the user to proceed or abort
    prompt("Proceed with resolution?");
    let _ = std::io::stdout().flush();

    // Get the input
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();

    match line.trim() {
        "y" => {
            // Pull the entrier
            puller.pull();
        },
        _ => {
            note("aborted; nothing was written");
            return;
        }
    }
}
