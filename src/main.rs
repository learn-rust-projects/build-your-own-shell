#[allow(unused_imports)]
use std::io::{self, Write};

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
            let command_type = command_trim.strip_prefix("type ").unwrap();
            if command_type == "exit" || command_type == "echo" || command_type == "type" {
                println!("{} is a shell builtin", command_type);
            }else{
                println!("{command_type}: not found");
            }
        }else{
            println!("{}: command not found", command_trim);
        }
    }
}
