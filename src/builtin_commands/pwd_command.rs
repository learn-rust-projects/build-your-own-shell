use super::prelude::*;
/// Pwd命令处理器
pub struct PwdCommand;

impl Builtin for PwdCommand {
    fn execute(
        &self,
        _params: Vec<String>,
        _context: &mut ExecutionContext,
    ) -> BuiltinCommandResult {
        BuiltinCommandResult::new_with_stdout(
            std::env::current_dir()
                .context("pwd failed\n")
                .map(|dir| format!("{}\n", dir.display()))
                .unwrap_or("".to_string()),
        )
    }
}
