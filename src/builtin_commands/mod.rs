use std::{fs::File, process::Stdio};

use anyhow::Context;
use rustyline::{
    Editor,
    history::{FileHistory, History},
};

use crate::{CommandResult, auto_completion::MyCompleter};
mod cd_command;
mod echo_command;
mod exit_command;
mod history_command;
mod prelude;
mod pwd_command;
mod type_command;
pub use cd_command::CdCommand;
pub use echo_command::EchoCommand;
pub use exit_command::ExitCommand;
pub use history_command::HistoryCommand;
pub use pwd_command::PwdCommand;
pub use type_command::TypeCommand;

/// 内置命令接口
pub trait BuiltinCommand {
    fn execute(
        &self,
        params: Box<dyn Iterator<Item = String>>,
        rl: &mut Editor<MyCompleter, FileHistory>,
    ) -> CommandResult;
}

/// 内置命令工厂
pub struct BuiltinCommandFactory;

impl BuiltinCommandFactory {
    pub fn create_command(command: &str) -> Option<Box<dyn BuiltinCommand>> {
        match command.parse::<crate::BuildinCommand>() {
            Ok(crate::BuildinCommand::Exit) => Some(Box::new(ExitCommand)),
            Ok(crate::BuildinCommand::Echo) => Some(Box::new(EchoCommand)),
            Ok(crate::BuildinCommand::Type) => Some(Box::new(TypeCommand)),
            Ok(crate::BuildinCommand::Pwd) => Some(Box::new(PwdCommand)),
            Ok(crate::BuildinCommand::Cd) => Some(Box::new(CdCommand)),
            Ok(crate::BuildinCommand::History) => Some(Box::new(HistoryCommand)),
            _ => None,
        }
    }
}
