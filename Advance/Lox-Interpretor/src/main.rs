#![allow(unused_variables)]
use std::{env, process::ExitCode};
mod scanner;
mod token_type;
use crate::scanner::ScanError;

pub mod lox {
    use crate::scanner::{self, ScanError};
    use crate::token_type::Literal;
    use std::fs;

    static HAD_ERROR: bool = false;
    pub fn error(line: u32, message: char) {
        report(1, &"".to_string(), message)
    }

    fn report(line: u32, whr: &String, message: char) {
        eprintln!("[line {line}] Error: Unexpected character: {}", message)
    }

    pub fn run_file(path: &String) -> Result<(), Vec<ScanError>> {
        if HAD_ERROR {}
        let file_contents = fs::read_to_string(path).expect("Failed to read the file");
        run(file_contents)
    }

    fn run(source: String) -> Result<(), Vec<ScanError>> {
        let mut scanner = scanner::Scanner::new(source);
        let (tokens, errors) = scanner.scan_tokens();

        for tkn in tokens {
            let token_typ = tkn.token_type;
            let lexeme = tkn.lexeme;
            match tkn.literal {
                Some(val) => match val {
                    Literal::Number(val) => {
                        if val.fract() == 0.0 {
                            println!("{:?} {} {:.1}", token_typ, lexeme, val);
                        } else {
                            println!("{:?} {} {}", token_typ, lexeme, val);
                        }
                    }
                    Literal::String(val) => {
                        println!("{:?} {} {}", token_typ, lexeme, val);
                    }
                    Literal::Nil => {}
                },
                None => {
                    println!("{:?} {} null", token_typ, lexeme);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::from(64);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            eprintln!("Logs from your program will appear here!");
            match lox::run_file(filename) {
                Ok(_) => ExitCode::SUCCESS,
                Err(errors) => {
                    // eprintln!("{:#?}",err);
                    for err in errors {
                        match err {
                            ScanError::UnexpectedCharacter { line, ch } => {
                                eprintln!("[line {}] Error: Unexpected character: {}", line, ch);
                            }
                            ScanError::UnterminatedString { line } => {
                                eprintln!("[line {}] Error: Unterminated string.", line);
                            }
                        }
                    }
                    ExitCode::from(65)
                }
            }
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::FAILURE;
        }
    }
}
