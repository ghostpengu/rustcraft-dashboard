use std::io::{ self, Write };
use rustyline::error::ReadlineError;
use rustyline::{ DefaultEditor, Result };
use std::process::exit;
use std::thread;
use crate::database;
use database::*;

use std::process::Command;
enum Tokens {
    Print(String),
    CreateUser(String, String,String),
    DeleteUser(String),
    Read(String),
    ReadAll,
    Delete,
    Clear,
    Exit,
    Unknown,
}

fn parse_command(input: &str) -> Tokens {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.len() {
        1 if parts[0].to_lowercase() == "exit" => Tokens::Exit,
        2 if parts[0].to_lowercase() == "deleteuser" => Tokens::DeleteUser(parts[1].to_string()),
        1 if parts[0].to_lowercase() == "readall" => Tokens::ReadAll,
        1 if parts[0].to_lowercase() == "delete" => Tokens::Delete,
        2 if parts[0].to_lowercase() == "read" => Tokens::Read(parts[1].to_string()),
        1.. if parts[0].to_lowercase() == "print" => Tokens::Print(parts[1..].join(" ")),
        4 if parts[0].to_lowercase() == "createuser" =>
            Tokens::CreateUser(parts[1].to_string(), parts[2].to_string(),parts[3].to_string()),
        1 if parts[0].to_lowercase() == "clear" => Tokens::Clear,
        _ => Tokens::Unknown,
    }
}
pub fn console() {
    thread::spawn(move || {
        let mut rl = DefaultEditor::new().unwrap();

        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    match parse_command(&line) {
                        Tokens::CreateUser(user, pass,core) => {
                            let passclone = pass.clone();
                            let usernameclone = user.clone();
                            let user = User {
                                username: user,
                                password: pass,
                                token: generate_token(usernameclone, passclone).to_string(),
                                cores:core
                            };

                            match Database::writedata(&user) {
                                Ok(suc) => println!("Created user {:?}", &user),
                                Err(err) => println!("{err}"),
                            }
                        }
                        Tokens::ReadAll => {
                            let data = Database::readdatabase();
                            println!("{:?}", data);
                        }
                        Tokens::Read(reading) => {
                            let user = Database::dataread(&reading);
                            println!("{:?}", user);
                        }
                        Tokens::Clear => {
                            Command::new("clear").status().expect("Failed to clear");
                        }
                        Tokens::Print(txt) => {
                            println!("{txt}");
                        }
                        Tokens::DeleteUser(name) => {
                            match Database::deleteuser(name) {
                                Ok(_) => println!("Deleted user"),
                                Err(err) => println!("Error: {err}"),
                            }
                            // Implement deletion logic here
                        }
                        Tokens::Delete => {
                            match Database::deletedata() {
                                Ok(_) => println!("Deleted database💀"),
                                Err(err) => println!("Error: {err}"),
                            }
                            // Implement deletion logic here
                        }
                        Tokens::Exit => {
                            println!("Exiting...");
                            exit(0);
                        }
                        Tokens::Unknown => {
                            println!("Unknown Command");
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Exiting....");
                    exit(0);
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    });
}
