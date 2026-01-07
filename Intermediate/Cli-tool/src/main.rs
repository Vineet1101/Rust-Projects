#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{env, fs, os::unix::fs::PermissionsExt, path::Path};

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode();
        metadata.is_file() && (mode & 0o111 != 0)
    } else {
        false
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        const EXIT_CMD: &str = "exit";
        const ECHO_CMD: &str = "echo";
        const TYPE_CMD: &str = "type";
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readLine");
        let some = guess.trim().split(' ').collect::<Vec<&str>>();
        let cmd = some[0];
        match cmd {
            EXIT_CMD => {
                return;
            }
            ECHO_CMD => {
                for i in some {
                    if i == "echo" {
                        continue;
                    }
                    print!("{} ", i);
                }
                print!("\n");
            }
            TYPE_CMD => match some[1] {
                ECHO_CMD | EXIT_CMD | TYPE_CMD => {
                    println!("{} is a shell builtin", some[1]);
                }
                _ => {
                    let mut found = false;
                    if let Some(path_var) = env::var_os("PATH") {
                        for dir in env::split_paths(&path_var) {
                            let full_path = dir.join(some[1]);

                            if is_executable(&full_path) {
                                println!("{} is {}", some[1], full_path.display());
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        println!("{}: not found", some[1]);
                    }
                }
            },
            _ => {
                let mut found = false;
                if let Some(path_var) = env::var_os("PATH") {
                    for dir in env::split_paths(&path_var) {
                        let full_path = dir.join(some[0]);

                        if is_executable(&full_path) {
                            // println!("Program was passed {} args (including program name).", some.len());
                            let mut child=Command::new(&some[0])
                                .args(some.split_at(1).1)
                                .spawn()
                                .expect("failed to execute the process");
                            child.wait().expect("failed to wait spawned process");
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    println!("{}: command not found", guess.trim());
                }
            }
        }
    }
}
