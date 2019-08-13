use std::io::{self, Write};
use std::env;
use std::fs;
use std::path::Path;
use std::process::{{Command, exit}};
use colored::*;

fn make_prompt() -> colored::ColoredString {
    let username = "USERNAME";
    let mut prompt = String::new();
    match env::var(username) {
        Ok(value) => {
            let value = value.green();
            prompt.push_str(&(value));
        },
        Err(_) => {
            prompt.push_str("$");
        }
    }
    prompt.bold()
}

fn check_command_availability(binary: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, binary);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}


fn execute_command(binary: &str, args: &str) {
    if !check_command_availability(binary) {
        println!("commnand not found in PATH");
        return;
    }
    let mut child = Command::new(binary)
                        .args(args.split_whitespace())
                        .spawn()
                        .expect("failed to run command");
    let ecode = child.wait()
                     .expect("failed to wait on child");

    if !ecode.success() {
        return ;
    }
}


fn split_multiple_commands(input: &str) -> Vec<&str> {
    input.split(';').collect()
}


fn change_directory(dir: &str) {
    let path = Path::new(dir);
    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("{}", e);
    }
}


fn export_to_env(args: &str) {
    let (envvar, value) = split_first(args, '=');
    let value = &value[1..value.len()];
    env::set_var(envvar, value);
}


fn exit_program() {
    println!("Thanks for using seash!");
    exit(0x0100);
}


fn execute_builtin(cmd: &str, args: &str) {
    match cmd {
        "cd"       => change_directory(args),
        "export"   => export_to_env(args),
        "exit" | _ => exit_program(),
    }
}

fn split_first(cmd: &str, cmp: char) -> (&str, &str) {
    let mut count = 0;
    for c in cmd.trim().to_string().chars() {
        if c == cmp {
            return (&cmd[0..count].trim(), &cmd[count..cmd.len()].trim())
        }
        count = count + 1;
    }
    (cmd.trim(), "")
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    let prompt = make_prompt();

    loop {
        input.clear();
        print!("{}> ", prompt);
        match io::stdout().flush() {
            Ok(status) => status,
            Err(_) => continue,
        }
        stdin.read_line(&mut input).unwrap();
        if input.len() <= 1 {
            continue;
        }
        let parsed_commands = split_multiple_commands(input.trim());

        for command in parsed_commands {
            let (binary, args) = split_first(command, ' ');
            match binary {
                "cd" | "exit" | "export" => execute_builtin(binary, args),
                _                        => execute_command(binary, args),
            }
        }
    }
}
