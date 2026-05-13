mod builtin_command_handler;
mod external_command_handler;
pub mod pipe_handler;
pub mod prelude;

use std::fs::File;

use crate::{
    builtin_commands::{BuiltinCommand, BuiltinCommandResult, Job},
    lexer::RedirectOp,
    parse::{Command, ExecutionContext, RedirectTarget}, // 添加ExecutionContext导入
};

/// 执行命令
pub fn execute_command(
    command: &Command,
    context: &mut ExecutionContext,
) -> anyhow::Result<CommandResult> {
    // 处理重定向
    apply_redirections(command, context)?;

    // 执行命令 提取为内部的闭包

    if command.argv.is_empty() {
        return Ok(CommandResult::default());
    }
    // 初始化后台作业
    init_job(context, command);
    // 使用简化的命令处理器
    let handler = crate::CommandHandlerFactory::create_handler(&command.argv[0]);

    let result = handler.execute(&command.argv[0], command.argv[1..].to_vec(), context);

    Ok(result)
}

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

/// 表示一个命令执行结果
#[derive(Debug)]
pub struct CommandResult {
    #[allow(dead_code)]
    pub exit_code: i32, // 退出码，0表示成功
    pub child: Option<std::process::Child>,
}
impl Default for CommandResult {
    fn default() -> Self {
        Self::new(0)
    }
}
impl CommandResult {
    pub fn new(exit_code: i32) -> Self {
        Self {
            exit_code,
            child: None,
        }
    }
    pub fn external_with_child(child: std::process::Child) -> Self {
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

/// 应用重定向
fn apply_redirections(command: &Command, context: &mut ExecutionContext) -> anyhow::Result<()> {
    for redirection in &command.redirections {
        match redirection.op {
            RedirectOp::Out | RedirectOp::OutAppend => {
                let fd = redirection.src_fd.unwrap_or(1); // 默认stdout
                if fd == 1 {
                    if let RedirectTarget::File(filename) = &redirection.target {
                        let file = File::options()
                            .write(true)
                            .create(true)
                            .append(redirection.op == RedirectOp::OutAppend)
                            .open(filename)?;
                        context.stdout = Some(file);
                    }
                } else if fd == 2
                    && let RedirectTarget::File(filename) = &redirection.target
                {
                    let file = File::options()
                        .write(true)
                        .create(true)
                        .append(redirection.op == RedirectOp::OutAppend)
                        .open(filename)?;
                    context.stderr = Some(file);
                }
            }
            RedirectOp::In => {
                let fd = redirection.src_fd.unwrap_or(0); // 默认stdin
                if fd == 0
                    && let RedirectTarget::File(filename) = &redirection.target
                {
                    let file = File::open(filename)?;
                    context.stdin = Some(file);
                }
            }
            RedirectOp::DupOut => {
                // 处理文件描述符复制：2>&1
                if let (Some(src_fd), RedirectTarget::Fd(target_fd)) =
                    (redirection.src_fd, &redirection.target)
                {
                    // 这里需要更复杂的文件描述符复制逻辑
                    // 简化实现：如果是stdout重定向到stderr或反之
                    if src_fd == 2 && *target_fd == 1 {
                        // stderr重定向到stdout
                        context.stderr = context.stdout.take();
                    } else if src_fd == 1 && *target_fd == 2 {
                        // stdout重定向到stderr
                        context.stdout = context.stderr.take();
                    }
                }
            }
            RedirectOp::Heredoc => {
                // 处理heredoc重定向
                if let RedirectTarget::Heredoc(_content) = &redirection.target {
                    // // 创建临时文件或管道来传递heredoc内容
                    // // 简化实现：使用临时文件
                    // let temp_file = tempfile::NamedTempFile::new()?;
                    // std::fs::write(temp_file.path(), content)?;
                    // let file = File::open(temp_file.path())?;
                    // context.stdin = Some(file.as_raw_fd());
                }
            }
            _ => {
                // 其他重定向操作符的实现在此省略，可根据需要添加
            }
        }
    }
    Ok(())
}

/// 初始化后台作业
fn init_job(context: &mut ExecutionContext, command: &Command) {
    let job = if context.background {
        let mut arg = command.argv.clone().join(" ");
        arg.push(' ');
        Some(Job::run(arg))
    } else {
        None
    };
    context.job = job;
}
