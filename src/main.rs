#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
    vec,
};

use anyhow::Context;
use is_executable::IsExecutable;

static GLOBAL_VEC: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let path = std::env::var("PATH").unwrap_or("".to_string());
    std::env::split_paths(&std::ffi::OsStr::new(&path)).collect::<Vec<_>>()
});

static HOME_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOME").unwrap_or("".to_string()));
/// 表示一个命令行参数
#[derive(Debug, Clone)]
struct Token {
    content: Vec<String>,
    redirect: Option<String>,
    redirect_err: Option<String>,
    append_out: bool,
    append_err: bool,
}
impl Token {
    fn new() -> Self {
        Self {
            content: vec![],
            redirect: None,
            redirect_err: None,
            append_out: false,
            append_err: false,
        }
    }
    fn add_content(&mut self, content: String) {
        self.content.push(content);
    }
    fn set_redirect(&mut self, redirect: String, append: bool) -> anyhow::Result<()> {
        let file = OpenOptions::new()
            .append(append)
            .create(true)
            .write(true)
            .open(&redirect);
        self.redirect = Some(redirect);
        Ok(())
    }
    fn set_redirect_err(&mut self, redirect_err: String, append: bool) -> anyhow::Result<()> {
        OpenOptions::new()
            .append(append)
            .create(true)
            .write(true)
            .open(&redirect_err)?;
        self.redirect_err = Some(redirect_err);
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .context("read line failed")?;

        let line_trim = line.trim();
        let tokens = split_quotes(line_trim);
        let iter = tokens.into_iter();
        for token in iter {
            let last_result = handle(token.content.into_iter());

            match token.redirect_err {
                None => {
                    if !last_result.stderr.is_empty() {
                        eprint!("{}", last_result.stderr);
                    }
                }
                Some(redirect_err) => {
                    let mut f = OpenOptions::new()
                        .append(token.append_err)
                        .write(true)
                        .open(redirect_err)?;
                    std::io::Write::write_all(&mut f, last_result.stderr.as_bytes())
                        .context("write file failed")?;
                }
            }
            match token.redirect {
                None => {
                    if !last_result.stdout.is_empty() {
                        print!("{}", last_result.stdout);
                    }
                }
                Some(redirect) => {
                    let mut f = OpenOptions::new()
                        .append(token.append_out)
                        .write(true)
                        .open(redirect)?;
                    std::io::Write::write_all(&mut f, last_result.stdout.as_bytes())
                        .context("write file failed")?;
                }
            }
        }
    }
}
/// 表示一个命令执行结果
#[derive(Debug, Clone)]
struct CommandResult {
    stdout: String, // 标准输出
    #[allow(dead_code)]
    stderr: String, // 标准错误
    #[allow(dead_code)]
    exit_code: i32, // 退出码，0表示成功
}

fn handle(mut params: impl Iterator<Item = String>) -> CommandResult {
    let command = params.next().context("command is empty");
    let command = match command {
        Ok(command) => command,
        Err(e) => {
            return CommandResult {
                stdout: "".to_string(),
                stderr: e.to_string(),
                exit_code: 1,
            };
        }
    };

    match command.as_str() {
        "exit" => std::process::exit(0),
        "echo" => CommandResult {
            stdout: format!("{}\n", params.collect::<Vec<_>>().join(" ")),
            stderr: "".to_string(),
            exit_code: 0,
        },
        "type" => {
            let command_type = params.next().context("type command is empty");
            let command_type = match command_type {
                Ok(command_type) => command_type,
                Err(e) => {
                    return CommandResult {
                        stdout: "".to_string(),
                        stderr: e.to_string(),
                        exit_code: 1,
                    };
                }
            };
            // TODO:使用 enum 优化
            match command_type.as_str() {
                "exit" | "echo" | "type" | "pwd" | "cd" => CommandResult {
                    stdout: format!("{} is a shell builtin\n", command_type),
                    stderr: "".to_string(),
                    exit_code: 0,
                },
                _ => match find_executable_file_in_paths(&command_type, &GLOBAL_VEC) {
                    Some(file_path) => CommandResult {
                        stdout: format!("{} is {}\n", command_type, file_path.display()),
                        stderr: "".to_string(),
                        exit_code: 0,
                    },
                    None => CommandResult {
                        stdout: "".to_string(),
                        stderr: format!("{command_type}: not found\n"),
                        exit_code: 1,
                    },
                },
            }
        }
        "pwd" => CommandResult {
            stdout: std::env::current_dir()
                .context("pwd failed\n")
                .map(|dir| format!("{}\n", dir.display()))
                .unwrap_or("".to_string()),
            stderr: "".to_string(),
            exit_code: 0,
        },
        "cd" => {
            let dir = params.next().context("cd command is empty\n");
            let dir = match dir {
                Ok(dir) => dir,
                Err(_) => {
                    return CommandResult {
                        stdout: "".to_string(),
                        stderr: "cd: missing operand\n".to_string(),
                        exit_code: 1,
                    };
                }
            };

            if params.next().is_some() {
                CommandResult {
                    stdout: "".to_string(),
                    stderr: "bash: cd: too many arguments\n".to_string(),
                    exit_code: 1,
                }
            } else {
                let dir = if dir == "~" { &HOME_DIR } else { &dir };
                match std::env::set_current_dir(dir).context("cd failed\n") {
                    Ok(_) => CommandResult {
                        stdout: "".to_string(),
                        stderr: "".to_string(),
                        exit_code: 0,
                    },
                    Err(_) => CommandResult {
                        stdout: "".to_string(),
                        stderr: format!("cd: {}: No such file or directory\n", dir),
                        exit_code: 1,
                    },
                }
            }
        }
        _ => match find_executable_file_in_paths(&command, &GLOBAL_VEC) {
            Some(file_path) => {
                let file_name = file_path.file_name().context("file name is empty");
                if file_name.is_err() {
                    return CommandResult {
                        stdout: "".to_string(),
                        stderr: format!("{}: file name is empty\n", command),
                        exit_code: 1,
                    };
                }
                Command::new(file_name.as_ref().unwrap())
                    .args(params)
                    .output()
                    .map(|output| CommandResult {
                        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                        exit_code: output.status.code().unwrap_or(1),
                    })
                    .unwrap_or_else(|_| CommandResult {
                        stdout: "".to_string(),
                        stderr: format!("{}: failed to execute\n", command),
                        exit_code: 1,
                    })
            }
            None => CommandResult {
                stdout: "".to_string(),
                stderr: format!("{}: command not found\n", command),
                exit_code: 1,
            },
        },
    }
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

fn split_quotes(line: &str) -> Vec<Token> {
    let mut token = Token::new();
    let mut match_type = MatchType::Default;
    let mut string = String::new();
    let mut res = Vec::new();
    let mut redirect = false;
    let mut redirect_err = false;
    for ch in line.chars() {
        match match_type {
            MatchType::Default => match ch {
                ch if ch.is_whitespace() => {
                    if !string.is_empty() {
                        if redirect {
                            let _ = token.set_redirect(string.clone(), token.append_out);
                            redirect = false;
                        } else if redirect_err {
                            let _ = token.set_redirect_err(string.clone(), token.append_err);
                            redirect_err = false;
                        } else {
                            token.add_content(string.clone());
                        }
                        string = String::new();
                    }
                    continue;
                }
                '\'' => match_type = MatchType::SingleQuote,
                '"' => match_type = MatchType::DoubleQuote,
                '\\' => match_type = MatchType::Escaping,
                '>' => {              
                        if redirect {
                            token.append_out = true;
                            continue;
                        } else if redirect_err {
                            token.append_err = true;
                            continue;
                        }
                        
                    if !string.is_empty() {
                        if string.ends_with("1") {
                            string.pop();
                            redirect = true;
                            token.append_out = false;
                        } else if string.ends_with("2") {
                            string.pop();
                            redirect_err = true;
                            token.append_err = false;
                        }
                    } else {
                        redirect = true;
                        token.append_out = false;
                    }
                    if !string.is_empty() {
                        token.add_content(string.clone());
                    }
                    string = String::new();
                }
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
            MatchType::Escaping => {
                string.push(ch);
                match_type = MatchType::Default;
            }
        }
    }

    if redirect {
        let _ = token.set_redirect(string.clone(), token.append_out);
    } else if redirect_err {
        let _ = token.set_redirect_err(string.clone(), token.append_err);
    } else {
        token.add_content(string.clone());
    }
    res.push(token);
    res
}

#[test]
fn test_split_quotes() {
    let line = "echo 'test     script' 'hello''example' shell''world";
    let params = split_quotes(line).get(0).unwrap().content.clone();
    assert_eq!(
        params,
        vec!["echo", "test     script", "helloexample", "shellworld"]
    );

    let line = "echo world     test";
    let params = split_quotes(line).get(0).unwrap().content.clone();
    assert_eq!(params, vec!["echo", "world", "test"]);
}
