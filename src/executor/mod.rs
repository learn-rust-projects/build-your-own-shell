mod builtin_command_handler;
mod external_command_handler;
pub mod prelude;

use crate::CommandResult;
use crate::{
    builtin_commands::BuiltinCommand,
    parse::ExecutionContext, // 添加ExecutionContext导入
};
/// 简化的命令处理器接口
pub trait CommandHandler {
    fn execute(
        &self,
        command: &str,
        args: Vec<String>,
        context: &mut ExecutionContext,
    ) -> CommandResult;
}

use builtin_command_handler::BuiltinCommandHandler;
use external_command_handler::ExternalCommandHandler;
/// 命令处理器工厂
pub struct CommandHandlerFactory;

impl CommandHandlerFactory {
    pub fn create_handler(command: &str) -> Box<dyn CommandHandler + 'static> {
        match command.parse::<BuiltinCommand>() {
            Ok(_) => Box::new(BuiltinCommandHandler),
            Err(_) => Box::new(ExternalCommandHandler),
        }
    }
}

use std::os::{
    fd::AsRawFd,
    unix::io::{IntoRawFd, OwnedFd},
};

use libc::write;
pub fn write_rawfd(fd: OwnedFd, data: &[u8]) {
    unsafe {
        let mut written = 0;
        while written < data.len() {
            let n = write(
                fd.as_raw_fd(),
                data[written..].as_ptr() as *const _,
                data.len() - written,
            );
            if n <= 0 {
                panic!("write failed");
            }
            written += n as usize;
        }
    }
}
