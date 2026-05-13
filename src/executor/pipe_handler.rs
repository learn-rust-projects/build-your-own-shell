use std::{fs::File, os::fd::FromRawFd};

use crate::{
    executor::{CommandResult, execute_command},
    parse::{Command, ExecutionContext},
};

/// 执行管道命令
pub fn execute_pipeline(
    commands: &[Command],
    context: &mut ExecutionContext,
) -> anyhow::Result<CommandResult> {
    if commands.is_empty() {
        return Ok(CommandResult::default());
    }

    let mut vec = vec![];

    for (i, command) in commands.iter().enumerate() {
        let is_last = i == commands.len() - 1;
        // 设置管道
        if !is_last {
            let mut fds = [0; 2];
            unsafe { libc::pipe(fds.as_mut_ptr()) };

            let reader = unsafe { File::from_raw_fd(fds[0]) };
            let writer = unsafe { File::from_raw_fd(fds[1]) };

            context.stdout = Some(writer);

            // 执行当前命令
            let mut command_context = ExecutionContext {
                stdin: context.stdin.take(),
                stdout: context.stdout.take(),
                stderr: context.stderr.take(),
                background: false,
                job: None,
            };
            let result = execute_command(command, &mut command_context)?;

            context.stdin = Some(reader);
            context.stdout = Some(unsafe { File::from_raw_fd(libc::dup(1)) });
            context.stderr = Some(unsafe { File::from_raw_fd(libc::dup(2)) });

            vec.push(result);
        } else {
            // 最后一个命令
            let result = execute_command(command, context)?;

            vec.push(result);
        }
    }
    let mut last_exit_code = 0;
    for result in vec {
        if let Some(mut child) = result.child {
            let status = child.wait()?;
            last_exit_code = status.code().unwrap_or(1);
        } else {
            last_exit_code = 0;
        }
    }
    Ok(CommandResult::new(last_exit_code))
}

pub fn excuete_single_command(
    command: &Command,
    context: &mut ExecutionContext,
) -> anyhow::Result<CommandResult> {
    let mut res = execute_command(command, context)?;
    if context.background {
        return Ok(CommandResult::default());
    }
    Ok(CommandResult::new(exit_code_by_child(res.child.take())))
}

fn exit_code_by_child(child: Option<std::process::Child>) -> i32 {
    child.map_or(0, |mut c| c.wait().ok().and_then(|e| e.code()).unwrap_or(1))
}
