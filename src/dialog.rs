use std::io::{self, IsTerminal};

pub fn prompt(prompt: &str) {
    if io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        eprintln!("\x1b[1;36m::\x1b[0m {prompt} [y/N]");
    } else {
        eprintln!("::{prompt} [y/N]");
    }
}

pub fn note(note: &str) {
    if io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        eprintln!("\x1b[1;33mnote\x1b[0m: {note}");
    } else {
        eprintln!("note: {note}");
    }
}

pub fn status(verb: &str, msg: &str) {
    if io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        eprintln!("\x1b[1;32m{verb:>12}\x1b[0m {msg}");
    } else {
        eprintln!("{verb:>12} {msg}");
    }
}

pub fn error(verb: &str, msg: &str) {
    if io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        eprintln!("\x1b[1;31m{verb:>12}\x1b[0m {msg}");
    } else {
        eprintln!("{verb:>12} {msg}");
    }
}