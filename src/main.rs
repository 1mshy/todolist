#[allow(unused)]
use std::env;
use std::fmt::Error;
use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
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
        eprintln!("INVALID ARGUMENT {}", $str);
        help!();
        exit(1);
    }
}

macro_rules! invalid_permissions {
    ($str:expr) => {
        eprintln!("INVALID PERMISSIONS {}", $str);
        exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = get_value_from_vector(&args, 1).unwrap_or_else(|_err| "");
    if (command.eq("") || args.len() <= 2) {
        invalid_arguments!(format!("Invalid value: {}", command));
    }
    let mut command_args = Vec::new();
    command_args.extend_from_slice(&args[2..args.len()]);
    match command {
        "add" => {
            add();
            done();
            add();
        }
        "done" => done(),
        _ => {
            invalid_arguments!(format!("Invalid value: {}", command));
        }
    }
}

fn get_value_from_vector(vector: &Vec<String>, index: usize) -> Result<&str, &str> {
    match vector.get(index) {
        Some(value) => Ok(value),
        None => Err("non existent"),
    }
}

fn add() {
    let content = file_content();
    println!("{}", content);
}

fn done() {
    let new_content = "why";
    let err = write_content(new_content);
    match err {
        Ok(()) => (),
        Err(e) => {
            println!("HI")
        }
    };
}

fn show() {}

fn file_content() -> String {
    let file_result = File::open("something");
    let mut file = file_result.unwrap_or_else(|err| {
        let create_file = File::create("something");
        match create_file {
            Ok(created_file) => created_file,
            Err(_) => {
                invalid_permissions!("Not able to create a file");
            }
        }
    });
    let mut content = String::new();
    file.read_to_string(&mut content).expect("TODO: panic message");
    return content;
}

fn write_content(content: &str) -> Result<(), std::io::Error> {
    let file_result = File::open("something");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => {
            File::create("something")?
        }
    };
    file.write_all(content.as_bytes())?;

    Ok(())
}
