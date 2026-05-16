use std::io::{self};

fn main(){
    println!("Welcome to the guessing game");
    println!("Enter your number: ");

    let mut input=String::new();

    io::stdin().read_line(&mut input).expect("A number is expected");

    println!("{input}");
}