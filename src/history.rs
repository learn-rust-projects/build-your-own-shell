use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::{Context, Result};
use rustyline::{
    Editor,
    history::{FileHistory, History},
};
static COUNTER: AtomicUsize = AtomicUsize::new(0);

use crate::auto_completion::MyCompleter;
pub fn handle_history_options(
    option: &str,
    file: Option<&String>,
    rl: &mut Editor<MyCompleter, FileHistory>,
) -> Result<()> {
    match option {
        "-r" => {
            let file_path = file.context("history: -r: missing operand")?;
            rl.load_history(&file_path)
                .context("history: -r: failed to load history")?;
            let len = rl.history().len();
            COUNTER.store(len, Ordering::SeqCst);
            Ok(())
        }
        "-w" => {
            let file_path = file.context("history: -w: missing operand")?;
            let history = rl
                .history()
                .iter()
                .fold(String::new(), |acc, line| acc + line + "\n");

            let mut file = File::create(file_path).context("history: -w: failed to create file")?;

            file.write_all(history.as_bytes())
                .context("history: -w: write failed")?;
            Ok(())
        }
        "-a" => {
            let file_path = file.context("history: -a: missing operand")?;
            let mut file = OpenOptions::new()
                .append(true)
                .open(file_path)
                .context("history: -a: failed to open file")?;

            let print = rl
                .history()
                .iter()
                .skip(COUNTER.load(Ordering::SeqCst))
                .fold(String::new(), |acc, line| acc + line + "\n");

            file.write_all(print.as_bytes())
                .context("history: -a: write failed")?;
            let history = rl.history();
            let len = history.len();
            COUNTER.store(len, Ordering::SeqCst);
            Ok(())
        }
        _ => Err(anyhow::anyhow!("history: invalid option: {}", option)),
    }
}

pub fn read_history_file(rl: &mut Editor<MyCompleter, FileHistory>) -> Result<()> {
    let path = std::env::var("HISTFILE").context("HISTFILE is not set");
    if let Ok(path) = path {
        handle_history_options("-r", Some(&path), rl)?
    }
    Ok(())
}

pub fn write_history_file(rl: &mut Editor<MyCompleter, FileHistory>) -> Result<()> {
    let path = std::env::var("HISTFILE").context("HISTFILE is not set")?;
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .context("history: -a: failed to open file")?;
    handle_history_options("-a", Some(&path), rl)?;
    Ok(())
}

pub fn print_iter(history: &FileHistory) -> impl Iterator<Item = String> {
    history
        .iter()
        .enumerate()
        .map(|(i, s)| format!("    {}  {s}\n", i + 1))
}
