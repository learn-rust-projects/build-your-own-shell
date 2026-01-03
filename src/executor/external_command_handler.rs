/// 外部命令处理器
use super::prelude::*;
pub struct ExternalCommandHandler;
use std::{
    os::{
        fd::AsRawFd,
        unix::io::{FromRawFd, RawFd},
    }
};

impl CommandHandler for ExternalCommandHandler {
    fn execute(
        &self,
        command: &str,
        args: Vec<String>,
        context: &mut ExecutionContext,
    ) -> CommandResult {
        let res = match crate::find_executable_file_in_paths(command, &crate::GLOBAL_VEC) {
            Some(file_path) => {
                let file_name = file_path.file_name().context("file name is empty");
                if file_name.is_err() {
                    eprintln!("{}: file name is empty\n", command);
                    return CommandResult::new(1);
                }
                let mut cmd = std::process::Command::new(file_name.unwrap());
                cmd.args(args);

                // 应用标准输入输出重定向
                if let Some(stdin) = context.stdin.take() {
                    
                        cmd.stdin(Stdio::from(stdin));
                    
                }
                if let Some(stdout) = context.stdout.take() {
             
                        cmd.stdout(Stdio::from(stdout));
                
                }
                if let Some(stderr) = context.stderr.take() {

                    cmd.stderr(Stdio::from(stderr));
                }
                let child = cmd.spawn().context("spawn command failed");
                match child {
                    Ok(child) => CommandResult::external_with_child(child),
                    Err(e) => {
                        eprintln!("{}: spawn command failed: {:?}\n", command, e);
                        CommandResult::new(1)
                    }
                }
            }
            None => {
                eprintln!("{}: command not found", command);
                CommandResult::new(1)
            }
        };
        res
    }
}
/// ⚠️ fd 必须是“新建 / dup / into_raw_fd”得到的
use std::os::unix::io::OwnedFd;

pub unsafe fn stdio_from_raw_fd(fd: OwnedFd) -> OwnedFd {
    let new_fd = unsafe { libc::dup(fd.as_raw_fd()) };
    unsafe { OwnedFd::from_raw_fd(new_fd) }
}
