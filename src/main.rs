#[allow(unused)]
use std::env;
use std::fs::File;
use std::process::exit;

macro_rules! help {
() => {
    println!("todolist add hello");
    println!("todolist done (numbers)");
    println!("todolist show");
    }
}
macro_rules! invalid_arguments {
    ($str:expr) => {
        eprintln!("INVALID PERMISSIONS: {}", $str);
        help!();
    }
}

macro_rules! invalid_permissions {
    ($str:expr) => {
        eprintln!("{}", $str);
        exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = match get_value_from_vector(&args, 1) {
        Ok(value) => value,
        Err(_err) => exit(1),
    };
    let mut command_args = Vec::new();
    command_args.extend_from_slice(&args[2..args.len()]);
    match command {
        "add" => add(),
        "done" => done(),
        _ => {invalid_arguments!(format!("Invalid value: {}", command));}
    }

    println!("{:?}", command_args);
    println!("HI THERE");
}
fn get_value_from_vector(vector: &Vec<String>, index:usize) -> Result<&str, &str> {
    match vector.get(index) {
        Some(value) => Ok(value),
        None => Err("non existent"),
    }
}

fn add() {
    file_content();
}
fn done() {

}
fn show() {

}

fn file_content() {
    let file_result = File::open("something");
    let file = match file_result {
        Ok(file) => file,
        Err(err) => {
            let create_file = File::create("something");
            match create_file {
                Ok(created_file) => created_file,
                Err(_) => {
                    invalid_permissions!("Not able to create a file");
                },
            }
        }
    };
    file.read

}