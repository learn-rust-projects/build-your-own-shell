#[allow(unused_imports)]
mod auto_completion;
mod builtin_commands;
mod executor;
mod history;
mod lexer;
mod parse;
mod utils;
use std::{path::PathBuf, sync::LazyLock};

use auto_completion::MyCompleter;
use executor::CommandHandlerFactory;
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
                let _ = parse_and_handle_line(&line, &mut rl);
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
    let _ = match command_type {
        CommandType::Simple(command) => excuete_single_command(&command, &mut context)?,
        CommandType::Pipeline(commands) => execute_pipeline(&commands, &mut context)?,
    };

    Ok(())
}
