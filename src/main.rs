#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

use anyhow::Context;
use is_executable::IsExecutable;

static GLOBAL_VEC: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let path = std::env::var("PATH").unwrap_or("".to_string());
    std::env::split_paths(&std::ffi::OsStr::new(&path)).collect::<Vec<_>>()
});

static HOME_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOME").unwrap_or("".to_string()));

fn main() -> anyhow::Result<()> {
    'outer: loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .context("read line failed")?;

        let line_trim = line.trim();

        let mut params = split_quotes(line_trim);
        let command = params.next().context("command is empty")?;

        match command.as_str() {
            "exit" => break 'outer,
            "echo" => println!("{}", params.collect::<Vec<_>>().join(" ")),
            "type" => {
                let command_type = params.next().context("type command is empty")?;
                // TODO:使用 enum 优化
                match command_type.as_str() {
                    "exit" | "echo" | "type" | "pwd" | "cd" => {
                        println!("{} is a shell builtin", command_type)
                    }
                    _ => match find_executable_file_in_paths(&command_type, &GLOBAL_VEC) {
                        Some(file_path) => println!("{} is {}", command_type, file_path.display()),
                        None => println!("{command_type}: not found"),
                    },
                }
            }
            "pwd" => println!(
                "{}",
                std::env::current_dir().context("pwd failed")?.display()
            ),

            "cd" => {
                let dir = params.next().context("cd command is empty")?;
                if params.next().is_some() {
                    println!("bash: cd: too many arguments");
                    break 'outer;
                }
                let dir = if dir == "~" { &HOME_DIR } else { &dir };
                match std::env::set_current_dir(dir).context("cd failed") {
                    Ok(_) => {}
                    Err(_) => println!("cd: {}: No such file or directory", dir),
                }
            }
            _ => match find_executable_file_in_paths(&command, &GLOBAL_VEC) {
                Some(file_path) => {
                    let _ = Command::new(file_path.file_name().context("file name is empty")?)
                        .args(params)
                        .status()
                        .context("failed to execute")?;
                }
                None => println!("{}: command not found", command),
            },
        }
    }
    Ok(())
}

fn find_executable_file_in_path(executable_file: &str, path: &Path) -> Option<PathBuf> {
    let file_path = path.join(executable_file);
    if file_path.is_file() && file_path.is_executable() {
        return Some(file_path);
    }
    None
}

fn find_executable_file_in_paths(executable_file: &str, paths: &Vec<PathBuf>) -> Option<PathBuf> {
    for path in paths {
        if (path.exists() || path.is_dir())
            && let Some(file_path) = find_executable_file_in_path(executable_file, path)
        {
            return Some(file_path);
        }
    }
    None
}
enum MatchType {
    Default,
    DoubleQuote,
    SingleQuote,
    Escaping,
    DoubleQuoteEscaping,
}

fn split_quotes(line: &str) -> impl Iterator<Item = String> {
    let mut params = Vec::new();
    let mut string = String::new();
    let mut match_type = MatchType::Default;
    for ch in line.chars() {
        match match_type {
            MatchType::Default => match ch {
                ch if ch.is_whitespace() => {
                    if !string.is_empty() {
                        params.push(string.clone());
                        string = String::new();
                    }
                    continue;
                }
                '\'' => match_type = MatchType::SingleQuote,
                '"' => match_type = MatchType::DoubleQuote,
                '\\' => match_type = MatchType::Escaping,
                _ => string.push(ch),
            },
            MatchType::SingleQuote => match ch {
                '\'' => match_type = MatchType::Default,
                _ => string.push(ch),
            },
            MatchType::DoubleQuote => match ch {
                '"' => match_type = MatchType::Default,
                '\\' => match_type = MatchType::DoubleQuoteEscaping,
                _ => string.push(ch),
            },
            MatchType::DoubleQuoteEscaping => match ch {
                '"' => {
                    string.push(ch);
                    match_type = MatchType::DoubleQuote;
                }
                '\\' => {
                    string.push(ch);
                    match_type = MatchType::DoubleQuote;
                }
                _ => {
                    string.push('\\');
                    string.push(ch);
                    match_type = MatchType::DoubleQuote;
                }
            },
            MatchType::Escaping => match ch {
                _ => {
                    string.push(ch);
                    match_type = MatchType::Default;
                }
            },
        }
    }
    if !string.is_empty() {
        params.push(string.clone());
    }
    params.into_iter()
}

#[test]
fn test_split_quotes() {
    let line = "echo 'test     script' 'hello''example' shell''world";
    let params = split_quotes(line).collect::<Vec<_>>();
    assert_eq!(
        params,
        vec!["echo", "test     script", "helloexample", "shellworld"]
    );

    let line = "echo world     test";
    let params = split_quotes(line).collect::<Vec<_>>();
    assert_eq!(params, vec!["echo", "world", "test"]);
}
