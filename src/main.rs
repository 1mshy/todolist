use std::fs::OpenOptions;
use std::{fs, io};
use std::io::Read;
use std::string::ToString;

const COMMAND_INDEX:usize = 1;
const EMPTY_COMMAND:&str = "";
const TODO_LIST_PATH:&str = "todo_list.txt";
fn main() {
    let args:Vec<String> = std::env::args().collect(); // arguments from the execution
    let command:String =  args.get(COMMAND_INDEX).map(|s| s.as_str()).unwrap_or( "").to_string();
    let mut content:String = String::new();
    read_list(&mut content).expect("TODO: panic message");
    println!("{}", content);
    match command.as_str() {
        "add" => {
            println!("ADD")
        },
        "show" => {
            println!("Stuff")
        }
        _ => {}
    }
}

fn read_list(content: &mut String) -> Result<(), io::Error> {
    let file_data = fs::read_to_string(TODO_LIST_PATH);

    match file_data {
        Ok(file_data) => {
            *content = file_data;
        },
        Err(_) => {
            std::process::exit(3);
        }
    }
    Ok(())
}