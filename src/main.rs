/*
    Made by BartuV
    github: https://github.com/BartuV
*/
#[allow(dead_code)]
mod lexpars;
mod create;
use colored::*;
use std::env;
use std::io;
use std::path::Path;
/*
commands

help:    shows how to use commands
compile: compiles to minecraft datapack
create:  creates a example datapack
*/
fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2{
        error("Invalid number of arguments")
    }else{
        if args[1] == "help"{
            println!("\n{}    {}\n{} {}\n{}  {}","help:".bold().green(),"shows this text","compile:".bold().green(),"compiles to minecraft datapack","create:".bold().green(),"create a example datapack")
        }else if args[1] == "compile"{
            let _location = get_input("Enter the pack location> ");
            if is_valid_path(&_location){
                lexpars::run(_location);
            }else{
                error("Invalid path.");
                return
            }
        }else if args[1] == "create"{
            let packname = get_input("Enter a pack name> ");
            let packversion = get_input("Enter a pack version> ");
            let _location = get_input("Enter a pack location> ");
            let num:i32 =  packversion.parse().unwrap();
            if is_valid_path(&_location){
                create::create(num, _location, packname); 
            }else{
                error("Invalid path.");
                return
            }
        }
    }
}

fn error(text: &str) {
    println!("{}{}","Command Line Error:".bold().red(), text.bold().red());
    std::process::exit(1);
}

fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

fn is_valid_path(path:&String) -> bool {
    Path::new(&path).exists()
}