#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::ExitCode;

mod scanner;
mod token_type;

pub mod Lox {
    static HAD_ERROR:bool=false;
    pub fn error(line:u32,message:char) {
        report(1,&"".to_string(),message)
    }
    
    fn report(line:u32,whr:&String,message:char){
        eprintln!("[line {line}] Error: Unexpected character: {}", message)
    }

    pub fn run_file(path: &String) {
        if HAD_ERROR{}
    }

    fn run(source: String) {
        // Scanner scanner=new Scanner(source);
        // let tokens =scanner.scanTokens();

        // for tkn in tokens{
        //     println!(tkn);
        // }
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    Lox::run_file(filename);

}
// {
//         match command.as_str() {
//         "tokenize" => {
//             // You can use print statements as follows for debugging, they'll be visible when running tests.
//             eprintln!("Logs from your program will appear here!");

//             let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
//                 eprintln!("Failed to read file {}", filename);
//                 String::new()
//             });

//             if !file_contents.is_empty() {
//                 eprintln!("{file_contents}");
//                 let mut flag = false;
//                 for chr in file_contents.chars() {
//                     match chr {
//                         '(' => {
//                             println!("LEFT_PAREN ( null");
//                         }
//                         ')' => {
//                             println!("RIGHT_PAREN ) null");
//                         }
//                         '{' => {
//                             println!("LEFT_BRACE {{ null");
//                         }
//                         '}' => {
//                             println!("RIGHT_BRACE }} null");
//                         }
//                         ',' => {
//                             println!("COMMA , null");
//                         }
//                         '.' => {
//                             println!("DOT . null")
//                         }
//                         '+' => {
//                             println!("PLUS + null");
//                         }
//                         '-' => {
//                             println!("MINUS - null");
//                         }
//                         '*' => {
//                             println!("STAR * null");
//                         }
//                         ';' => {
//                             println!("SEMICOLON ; null");
//                         }
//                         '=' => {}
//                         _ => {
//                             Lox::error(1,chr);
//                             flag = true;
//                         }
//                     }
//                 }

//                 if !flag {
//                     println!("EOF  null");
//                     return ExitCode::SUCCESS;
//                 } else {
//                     println!("EOF  null");
//                     return ExitCode::from(65);
//                 }
//             } else {
//                 println!("EOF  null");
//                 return ExitCode::SUCCESS;
//             }
//         }
//         _ => {
//             eprintln!("Unknown command: {}", command);
//             return ExitCode::FAILURE;
//         }
//     }
// }