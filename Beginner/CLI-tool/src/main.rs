//User should enter the repl
//take input from the terminal 
//with help command can see all the commands
//When incorrect commands are typed errors are shown
//need a loop that runs as long as users wants and to get out of the loop ctrl+c
//taking input from the terminal 
//matching the command the user gave with the existing commands if not matching throw error
//echo is simple so no use complicating things
//for cat command need to see how to interact with files in rust mainly how to read and write in a file
//for ls command i need to see the entire folder structure so how to see folder structure in rust
// simple looping over files and dire main point is to know the folder structure
//looping in a file just need to know how to read and write in a file using rust 


use ferris_says::say;
use std::{default, io::{BufWriter, stdin, stdout}};

fn echo(s:&String ){

    println!("{} it is entering only echo command",s);
}

fn cat(){

}

fn ls(){

}

fn find(){

}

fn grep(){

}

fn help(){

}

fn main() {
    println!("Enter the command('help' to see all commands)> ");
    loop{
        println!(">");
        let mut input=String::new();
        
        let echoCmd=String::from("echo");
        let catCmd=String::from("cat");
        let lsCmd=String::from("ls");
        let findCmd=String::from("find");
        let grepCmd=String::from("grep");
        let helpCmd=String::from("help");
        let exitCmd=String::from("exit");
        
        stdin().read_line( &mut input).expect("Please enter a valid command('help' to see all commands)");
        println!("input: {}",&input);
        match &input {
            catCmd=>{cat();}
            echoCmd =>{echo(&input);}
            lsCmd=>{ls();}
            findCmd=>{find();}
            grepCmd=>{grep();}
            helpCmd=>{help();}
            exitCmd=>{println!("Loops needs to end");break;}
            default=>{println!("{} Command not found",&input)}
        }

    }

}
