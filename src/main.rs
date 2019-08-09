use std::io::{self, Write};
use std::env;
use std::process;

fn make_prompt() -> String {
    let username = "USERNAME";
    let mut prompt = String::new();
    match env::var(username) {
        Ok(value) => {
            prompt.push_str(&(value + "@alterway"));
        },
        Err(_) => {
            prompt.push_str("$");
        }
    }
    prompt
}


fn execute_command(input: &str) {
    let output = process::Command::new("sh").arg("-c").arg(input)
                          .output()
                          .expect("failed to execute command");
    println!("\n{}", String::from_utf8_lossy(&output.stdout));
}


fn prepare_input(input: &str) -> Vec<&str> {
    input.split(';').collect()
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
        stdin.read_line(&mut input).expect("Failed to read user input");
        let command_list = prepare_input(input.trim());

        for command in command_list {
            if command == "exit" {
                println!("Thanks for using seash!");
                process::exit(0x0100);
            }

            execute_command(command);

        }
    }
}
