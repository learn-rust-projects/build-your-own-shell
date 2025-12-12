#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use is_executable::IsExecutable;
fn main() {
    // TODO: Uncomment the code below to pass the first stage
    // TODO: unwrap 使用context优化

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command_trim= command.trim();
        if command_trim == "exit" {
            break;
        }else if command_trim.starts_with("echo") {
            println!("{}", command_trim.strip_prefix("echo ").unwrap());
        }else if command_trim.starts_with("type") {
            let path = std::env::var("PATH").unwrap_or("".to_string());
            let paths: Vec<PathBuf> = std::env::split_paths(&std::ffi::OsStr::new(&path)).collect::<Vec<_>>();
            let command_type = command_trim.strip_prefix("type ").unwrap();
            if command_type == "exit" || command_type == "echo" || command_type == "type" {
                println!("{} is a shell builtin", command_type);
            }else if let Some(file_path) = find_executable_file_in_paths(command_type, paths) {
                let file_path = file_path.canonicalize().unwrap();
                println!("{} is {}", command_type, file_path.display());
            }else{
                println!("{command_type}: not found");
            }
        }else{
            println!("{}: command not found", command_trim);
        }
    }
}




fn find_executable_file_in_path(executable_file: &str, path: &Path) -> Option<PathBuf> {

    let file_path = path.join(executable_file);

    if file_path.is_file() && file_path.is_executable() {
        return Some(file_path);
    }

    None
}

fn find_executable_file_in_paths(executable_file: &str, paths: Vec<PathBuf>) -> Option<PathBuf> {
    for path in paths {
        if path.exists() || path.is_dir() {
            if let Some(file_path) = find_executable_file_in_path(executable_file, &path) {
                return Some(file_path);
            }
        }
    }
    None
}