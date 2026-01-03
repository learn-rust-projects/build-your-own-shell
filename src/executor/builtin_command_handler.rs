/// 内置命令处理器
use super::prelude::*;
pub struct BuiltinCommandHandler;

impl CommandHandler for BuiltinCommandHandler {
    fn execute(
        &self,
        command: &str,
        args: Vec<String>,
        context: &mut ExecutionContext,
    ) -> CommandResult {
        let middle_result = if let Some(cmd) = BuiltinFactory::create_command(command) {
            cmd.execute(args, context)
        } else {
            BuiltinCommandResult::new_with_stderr(format!("{}: command not found\n", command))
        };
        handler_middle_result(middle_result, context)
    }
}

fn handler_middle_result(
    middle_result: BuiltinCommandResult,
    context: &mut ExecutionContext,
) -> CommandResult {
    // 处理标准输出和错误输出
    if let Some(mut stdout) = context.stdout.take() {
        let _ = stdout.write_all(&middle_result.stdout);
    }
    if let Some(mut stderr) = context.stderr.take() {
        let _ = stderr.write_all(&middle_result.stderr);
    }
    middle_result.into()
}
