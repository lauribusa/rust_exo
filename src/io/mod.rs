use std::{env, fs};
use std::env::args;

pub fn write_file(){
    fs::write("database.txt", "Mes facultés mentales sont deux fois supérieures à ta cervelle de petit pois").expect("Failed to write");
}

pub fn read_file(){
    let txt = fs::read("database.txt");
    let mut text = "".to_string();
    match txt {
        Ok(content) => {
            let result = String::from_utf8(content);
            match result {
                Ok(s) => {
                    text = s;
                },
                Err(_) => {
                    eprintln!("Couldn't parse byte array");
                },
            }
        },
        Err(err) => {
            eprintln!("Couldn't read")
        },
    }

    // alternatively

    let txt = fs::read_to_string("database.txt");
    let mut text = String::new();
    match txt {
        Ok(s) => {
            text = s;
        },
        Err(e) => {
            eprintln!("Couldn't read file {}", e);
        },
    }
}

pub fn remove_file(){
    let result = fs::remove_file("database.txt");

    match result {
        Ok(_) => {
            println!("Successfully removed file.");
        },
        Err(e) => {
            eprintln!("Couldn't remove file. {}", e);
        },
    }
}

pub fn get_args(){
    // get arguments passed during cargo run
    let args = args();

    for arg in args {
        println!("{}", arg);
    }
}

pub fn get_dir(){
    let dir = env::current_dir();

    match dir {
        Ok(path) => {
            println!("Found path {:?}", path);
        },
        Err(e) => {
            eprintln!("Couldn't get path {}", e);
        },
    }
}

pub enum LogLevel {
    Error,
    Warning,
    Info
}

fn write_log_file(content: String, log_level: LogLevel){
    let mut log_content = String::new();
    
    let text = read_log_file();
    match text {
        Some(s) => {
            log_content = s;
        },
        None => (),
    }

    match log_level {
        LogLevel::Error => {
            log_content.push_str("ERROR: ");
        },
        LogLevel::Warning => {
            log_content.push_str("WARN: ");
        },
        LogLevel::Info => {
            log_content.push_str("INFO: ");
        },
    }

    log_content.push_str(content.as_str());
    log_content.push_str("\n");
    fs::write("logger.txt", log_content).expect("Failed to write");
}

pub fn write_info(content: String){
    write_log_file(content, LogLevel::Info);
}

pub fn write_warn(content: String){
    write_log_file(content, LogLevel::Warning);
}

pub fn write_err(content: String){
    write_log_file(content, LogLevel::Error);
}

fn read_log_file() -> Option<String> {
    let log_file = fs::read_to_string("logger.txt");

    match log_file {
        Ok(s) => {
            println!("{}", s);
            Some(s)
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        },
    }
}

pub fn print_log_content(){
    let result = read_log_file();
    println!("Printing...");
    match result {
        Some(s) => {
            println!("{}", s);
        },
        None => {
            println!("No content found.");
        },
    }
}