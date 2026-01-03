#[allow(unused_imports)]
mod auto_completion;
mod builtin_commands;
mod executor;
mod history;
mod lexer;
mod parse;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use auto_completion::MyCompleter;
use executor::CommandHandlerFactory;
use is_executable::IsExecutable;
use rustyline::{
    Editor,
    config::{CompletionType, Config, Configurer},
    error::ReadlineError,
    history::FileHistory,
};

use crate::parse::{
    CommandType, ExecutionContext, excuete_single_command, execute_pipeline, parse_command,
};

pub static GLOBAL_VEC: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let path = std::env::var("PATH").unwrap_or("".to_string());
    std::env::split_paths(&std::ffi::OsStr::new(&path)).collect::<Vec<_>>()
});
pub static HOME_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOME").unwrap_or("".to_string()));

fn main() -> anyhow::Result<()> {
    let config = Config::builder()
        .history_ignore_dups(false)?
        .completion_type(CompletionType::List) // 多候选时列出
        .bell_style(rustyline::config::BellStyle::Audible)               // 歧义时响铃
        .build();

    let completer = MyCompleter;
    let mut rl = Editor::with_config(config)?;
    rl.set_completion_type(rustyline::CompletionType::List);
    rl.set_helper(Some(completer));
    history::read_history_file(&mut rl)?;
    loop {
        match rl.readline("$ ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let res = parse_and_handle_line(&line, &mut rl);
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn parse_and_handle_line(
    line: &str,
    rl: &mut Editor<MyCompleter, FileHistory>,
) -> anyhow::Result<()> {
    let line_trim = line.trim();

    // 空行处理
    if line_trim.is_empty() {
        return Ok(());
    }

    // 词法分析
    let raw_tokens = crate::lexer::tokenize_line(line_trim)?;

    // 语法分析
    let command_type = parse_command(&raw_tokens);

    // 创建执行上下文
    let mut context = ExecutionContext::new(rl);

    // 执行命令
    let res = match command_type {
        CommandType::Simple(command) => excuete_single_command(&command, &mut context)?,
        CommandType::Pipeline(commands) => execute_pipeline(&commands, &mut context)?,
    };

    Ok(())
}

/// 表示一个命令执行结果
#[derive(Debug, Default)]
pub struct BuiltinCommandResult {
    stdout: Vec<u8>, // 标准输出
    #[allow(dead_code)]
    stderr: Vec<u8>, // 标准错误
    #[allow(dead_code)]
    exit_code: i32, // 退出码，0表示成功
}
impl BuiltinCommandResult {
    fn new_with_stdout(stdout: String) -> Self {
        Self {
            stdout: stdout.into_bytes(),
            ..Default::default()
        }
    }
    fn new_with_stderr(stderr: String) -> Self {
        Self {
            stderr: stderr.into_bytes(),
            exit_code: 1,
            ..Default::default()
        }
    }
}
/// 表示一个命令执行结果
#[derive(Debug)]
pub struct CommandResult {
    #[allow(dead_code)]
    exit_code: i32, // 退出码，0表示成功
    child: Option<std::process::Child>,
}
impl Default for CommandResult {
    fn default() -> Self {
        Self::new(0)
    }
}
impl CommandResult {
    fn new(exit_code: i32) -> Self {
        Self {
            exit_code,
            child: None,
        }
    }
    fn external_with_child(child: std::process::Child) -> Self {
        Self {
            exit_code: 0,
            child: Some(child),
        }
    }
}

impl From<BuiltinCommandResult> for CommandResult {
    fn from(value: BuiltinCommandResult) -> Self {
        Self {
            exit_code: value.exit_code,
            child: None,
        }
    }
}

fn find_executable_file_in_path(path: &Path) -> Option<PathBuf> {
    if path.is_file() && path.is_executable() {
        return Some(path.to_path_buf());
    }
    None
}

pub fn print_iter(history: &FileHistory) -> impl Iterator<Item = String> {
    history
        .iter()
        .enumerate()
        .map(|(i, s)| format!("    {}  {s}\n", i + 1))
}
fn find_executable_file_in_paths(executable_file: &str, paths: &Vec<PathBuf>) -> Option<PathBuf> {
    for path in paths {
        if (path.exists() || path.is_dir())
            && let Some(file_path) = find_executable_file_in_path(&path.join(executable_file))
        {
            return Some(file_path);
        }
    }
    None
}

use std::fs;

fn find_all_executable_file_in_paths(paths: &[PathBuf]) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|path| path.exists() && path.is_dir())
        .flat_map(|dir| {
            fs::read_dir(dir)
                .map(|rd| {
                    Box::new(
                        rd.filter_map(|entry| entry.ok())
                            .filter_map(|entry| find_executable_file_in_path(&entry.path())),
                    ) as Box<dyn Iterator<Item = PathBuf>>
                })
                .unwrap_or_else(|_| Box::new(std::iter::empty()))
        })
        .collect()
}
