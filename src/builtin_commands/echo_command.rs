use super::prelude::*;
/// Echo命令处理器
pub struct EchoCommand;

impl Builtin for EchoCommand {
    fn execute(
        &self,
        params: Vec<String>,
        _context: &mut ExecutionContext,
    ) -> BuiltinCommandResult {
        BuiltinCommandResult::new_with_stdout(format!("{}\n", params.join(" ")))
    }
}
