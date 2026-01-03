pub use std::{fs::File, process::Stdio};

pub use anyhow::Context;
pub use rustyline::{
    Editor,
    history::{FileHistory, History},
};

pub use crate::{CommandResult, auto_completion::MyCompleter, builtin_commands::BuiltinCommand};
