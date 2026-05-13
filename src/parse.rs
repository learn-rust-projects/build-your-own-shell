use std::fs::File;

use regex::Regex;

use crate::{
    builtin_commands::{GLOBAL_COMPLETION_DECLARE, Job},
    executor::CommandResult,
    lexer::{RawToken, RedirectOp},
};
#[derive(Debug, Clone)]
pub struct Command {
    pub argv: Vec<String>,
    pub redirections: Vec<Redirection>, // 有序，决定语义
}

#[derive(Debug, Clone)]
pub struct Redirection {
    pub src_fd: Option<u8>, // None = 默认 fd（>, <）
    pub op: RedirectOp,
    pub target: RedirectTarget,
}

#[derive(Debug, Clone)]
pub enum RedirectTarget {
    File(String), // > file
    Fd(u8),       // 2>&1
    Close,        // 2>&-
    #[allow(dead_code)]
    Heredoc(String),
}

#[derive(Debug, Clone)]
pub struct CommandGroup {
    pub commands: Vec<Command>,
    pub background: bool,
}

pub fn parse_command(tokens: &[RawToken]) -> Vec<CommandGroup> {
    let mut command_groups = Vec::new();
    let mut commands = Vec::new();
    let mut current_tokens = Vec::new();

    for token in tokens {
        match token {
            RawToken::Pipe => {
                if !current_tokens.is_empty() {
                    commands.push(parse_simple_command(&current_tokens));
                    current_tokens.clear();
                }
            }
            RawToken::Background => {
                if !current_tokens.is_empty() {
                    commands.push(parse_simple_command(&current_tokens));
                    current_tokens.clear();
                }
                command_groups.push(CommandGroup {
                    commands: std::mem::take(&mut commands),
                    background: true,
                });
            }
            _ => {
                current_tokens.push(token.clone());
            }
        }
    }

    // 处理最后一个命令
    if !current_tokens.is_empty() {
        commands.push(parse_simple_command(&current_tokens));
        command_groups.push(CommandGroup {
            commands,
            background: false,
        });
    }

    command_groups
}

pub fn parse_simple_command(tokens: &[RawToken]) -> Command {
    let mut argv = Vec::new();
    let mut redirections = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            RawToken::Word(w) => {
                let expanded = expand(w);
                if !expanded.is_empty() {
                    argv.push(expanded);
                }

                i += 1;
            }

            RawToken::IoNumber(fd) => {
                let src_fd = Some(*fd);

                match tokens.get(i + 1) {
                    Some(RawToken::Redirect(op)) => {
                        let target = parse_redirect_target(&tokens[i + 2]);
                        redirections.push(Redirection {
                            src_fd,
                            op: *op,
                            target,
                        });
                        i += 3;
                    }
                    _ => panic!("io number not followed by redirect"),
                }
            }

            RawToken::Redirect(op) => {
                let src_fd = None;
                let target = parse_redirect_target(&tokens[i + 1]);

                redirections.push(Redirection {
                    src_fd,
                    op: *op,
                    target,
                });
                i += 2;
            }

            _ => panic!("unexpected token"),
        }
    }

    Command { argv, redirections }
}

fn parse_redirect_target(token: &RawToken) -> RedirectTarget {
    match token {
        RawToken::Word(w) if w == "-" => RedirectTarget::Close,
        RawToken::Word(w) => match w.parse::<u8>() {
            Ok(fd) => RedirectTarget::Fd(fd),
            Err(_) => RedirectTarget::File(w.clone()),
        },
        _ => panic!("invalid redirect target"),
    }
}

use std::os::unix::io::FromRawFd;
/// 命令执行上下文
#[derive(Debug)]
pub struct ExecutionContext {
    pub stdin: Option<File>,
    pub stdout: Option<File>,
    pub stderr: Option<File>,
    pub background: bool,
    pub job: Option<usize>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            stdin: Some(unsafe { File::from_raw_fd(libc::dup(0)) }),
            stdout: Some(unsafe { File::from_raw_fd(libc::dup(1)) }),
            stderr: Some(unsafe { File::from_raw_fd(libc::dup(2)) }),
            background: false,
            job: None,
        }
    }
}


fn expand(input: &str) -> String {
    let env = &GLOBAL_COMPLETION_DECLARE.lock().unwrap().completions;

    let re = Regex::new(r"\$\{([a-zA-Z_][a-zA-Z0-9_]*)\}|\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

    re.replace_all(input, |caps: &regex::Captures| {
        // ${VAR}
        let name = caps
            .get(1)
            .or_else(|| caps.get(2))
            .map(|m| m.as_str())
            .unwrap_or("");

        env.get(name).cloned().unwrap_or_default()
    })
    .to_string()
}
