use super::prelude::*;
/// Exit命令处理器
pub struct ExitCommand;

impl Builtin for ExitCommand {
    fn execute(
        &self,
        _params: Vec<String>,
        _context: &mut ExecutionContext,
    ) -> BuiltinCommandResult {
        match crate::history::write_history_file(_context.rl) {
            Ok(_) => {
                std::process::exit(0);
            }
            Err(_) => {
                std::process::exit(0);
            }
        }
    }
}
