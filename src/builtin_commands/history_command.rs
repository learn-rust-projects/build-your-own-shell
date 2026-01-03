use super::prelude::*;
/// History命令处理器
pub struct HistoryCommand;

impl Builtin for HistoryCommand {
    fn execute(&self, params: Vec<String>, context: &mut ExecutionContext) -> BuiltinCommandResult {
        let mut params = params.iter();
        match params.next() {
            Some(dir) => {
                if dir == "-r" || dir == "-w" || dir == "-a" {
                    let file: Option<&String> = params.next();
                    return match crate::history::handle_history_options(dir, file, context.rl) {
                        Ok(_) => BuiltinCommandResult::default(),
                        Err(e) => BuiltinCommandResult::new_with_stderr(format!("{}\n", e)),
                    };
                }

                let num = dir
                    .parse::<usize>()
                    .context("history number is not a number");
                let history = context.rl.history();
                let len = history.len();
                match num {
                    Ok(num) => {
                        if num > len {
                            BuiltinCommandResult::new_with_stdout(
                                crate::print_iter(history).collect(),
                            )
                        } else {
                            BuiltinCommandResult::new_with_stdout(
                                crate::print_iter(history).skip(len - num).collect(),
                            )
                        }
                    }
                    Err(_) => BuiltinCommandResult::new_with_stderr(format!(
                        "history: {}: event not found\n",
                        dir
                    )),
                }
            }
            None => BuiltinCommandResult::new_with_stdout(
                crate::print_iter(context.rl.history()).collect(),
            ),
        }
    }
}
