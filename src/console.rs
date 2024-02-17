use std::io::{self, Write};

use std::process::exit;
use std::thread;
use crate::database;
use database::*;

use std::process::Command;
enum Tokens {
    Print(String),
    CreateUser(String,String),
    Read(String),
    ReadAll(),
    Delete(),
    Clear,
    Exit,
    Unknown,
}

fn parse_command(input: &str) -> Tokens {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    match parts.len() {
        1 if parts[0].to_lowercase() == "exit" => Tokens::Exit,
        1 if parts[0].to_lowercase() == "readall" => Tokens::ReadAll(),
        1 if parts[0].to_lowercase() == "delete" => Tokens::Delete(),
        2 if parts[0].to_lowercase() == "read" => Tokens::Read(parts[1].to_string()),
        1.. if parts[0].to_lowercase() == "print" => Tokens::Print(parts[1..].join(" ")),
        3 if parts[0].to_lowercase() == "createuser" => Tokens::CreateUser(parts[1].to_string(),parts[2].to_string()),
        1 if parts[0].to_lowercase() == "clear"=>Tokens::Clear,
        _ => Tokens::Unknown,
    }
}
pub fn console(){
    thread::spawn(move || {
        loop {
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
    
            match parse_command(&input) {
                Tokens::CreateUser(username,pass) => {
                    let clone = pass.clone();
                    let user = User{
                        username:username,
                        password:pass,
                        token:generate_token(clone).to_string()
                    };
                   
                    match Database::writedata(&user) {
                        Ok(suc)=>  println!("Created user {:?}",&user),
                        Err(err)=>println!("{err}")
                    }
                 }
                Tokens::ReadAll() => {
                   let data = Database::readdatabase();
                   println!("{:?}",data);
                }
                Tokens::Read(reading) => {
                    let user = Database::dataread(&reading);
                    println!("{:?}",user);
                }
                Tokens::Clear => {
                    Command::new("clear").status().expect("Failed to clear");      
                }
                Tokens::Print(txt) =>{
                    println!("{txt}");
                }
                Tokens::Delete() => {
                   match  Database::deletedata(){
                        Ok(_) => println!("Deleted database💀"),
                        Err(err) => println!("Error: {err}")
                   }
                    // Implement deletion logic here
                }
                Tokens::Exit => {
                    println!("Exiting...");
                    exit(0)
                }
                Tokens::Unknown => {
                    println!("Unknown Command");
                }
            }
        }
    });
}