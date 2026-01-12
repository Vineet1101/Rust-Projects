#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;


fn error(unexpected_char:char){
    eprintln!("[line 1] Error: Unexpected character: {}",unexpected_char)
}

fn main()->ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                eprintln!("{file_contents}");
                let mut flag=false;
                for chr in file_contents.chars(){
                    match chr{
                        '('=>{println!("LEFT_PAREN ( null");}
                        ')'=>{println!("RIGHT_PAREN ) null");}
                        '{'=>{println!("LEFT_BRACE {{ null");}
                        '}'=>{println!("RIGHT_BRACE }} null");}
                        ','=>{println!("COMMA , null");}
                        '.'=>{println!("DOT . null")}
                        '+'=>{println!("PLUS + null");}
                        '-'=>{println!("MINUS - null");}
                        '*'=>{println!("STAR * null");}
                        ';'=>{println!("SEMICOLON ; null");}
                        '='=>{
                            
                        }
                        _=>{
                            error(chr);
                            flag=true;
                        }
                    }
                }

                if !flag {
                    println!("EOF  null");
                    return ExitCode::SUCCESS}
                else {
                    println!("EOF  null");
                    return ExitCode::from(65);}
            } else {
                println!("EOF  null");
                return ExitCode::SUCCESS
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::FAILURE
        }
    }
}
