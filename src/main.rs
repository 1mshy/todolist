#![allow(unused)]
use std::env;
use std::{io};
use std::fs::{OpenOptions};
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
const FILE_PATH: &str = "todo_list.txt";

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
    let mut content = String::new();
    match file_content(&mut content) {
        Ok(())=>(),
        Err(_) => {

        }
    }
    println!("{}", content);
}

fn done() {
    let new_content = "why";
    let err = write_content(new_content);
    match err {
        Ok(()) => (),
        Err(e) => {
            println!("{}", e)
        }
    };
}

fn show() {}

fn file_content(content: &mut String) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(FILE_PATH)?;
    file.read_to_string(content)?;
    Ok(())
}

fn write_content(content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .truncate(true)
        .open(FILE_PATH)?;

    write!(file, "{}", content)?;
    Ok(())
}
