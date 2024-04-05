use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use crate::instance;
use instance::*;
use rusqlite::{ Connection, Result };
pub struct Database;
use sha256::digest;
#[derive(Serialize, Debug,Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub token: String,
    pub cores:String
}


pub fn generate_token(usename:String,pass: String) -> u64 {
    let hash = digest(format!("{usename}{pass}"));
     // Take the first 8 bytes of the hash and convert them to u64
     let hash_slice = &hash.as_bytes()[0..8];
     let mut token: u64 = 0;
     for &byte in hash_slice {
         token <<= 8;
         token |= byte as u64;
     }
    return token;
}
impl User {
    pub fn new(username: &str, password: &str, token: &str,cores:&str) -> Self {
        User {
            username: String::from(username),
            password: String::from(password),
            token: String::from(token),
            cores:String::from(cores)
        }
    }

    pub fn matchtoken(&self, tok: &String) -> bool {
        self.token == *tok
    }
    pub fn login(&self, username: &String, password: &String) -> bool {
        if self.username == *username && self.password == *password { true } else { false }
    }
}

impl Database {
    pub fn deleteuser(user:String) -> Result<()> {
        let conn = Connection::open("data.db").unwrap();
        let usr = Database::dataread(&user);
        let tok = usr.token;
        Instance::deletefolder(format!("minecraftdata/{tok}"));
        conn.execute("DELETE FROM users WHERE username = ?", &[&user])?;
        Ok(())
    }
    pub fn deletedata() -> Result<()> {
        let conn = Connection::open("data.db").unwrap();

        conn.execute("DELETE FROM users", [])?;
        Ok(())
    }
    pub fn dataread(username:&String) -> User{
        let default = User {
            username: "error reading database".to_string(),
            password: "error reading database".to_string(),
            token: "error reading database".to_string(),
            cores:"error reading database".to_string()
        };
        let conn = Connection::open("data.db").unwrap();
        let mut dat = conn.prepare("SELECT * FROM users").unwrap();
        let userdata = dat
            .query_map([], |row| {
                Ok(User {
                    username: row.get(0).unwrap(),
                    password: row.get(1).unwrap(),
                    token: row.get(2).unwrap(),
                    cores: row.get(3).unwrap()
                })
            })
            .unwrap();

        for usr in userdata {
            let h = usr.unwrap().clone();
            if &h.username == username{
                return h;
            }
        }
        return default;
    }

    pub fn readdatabase() -> Vec<User> {
        let conn = Connection::open("data.db").unwrap();
        let mut dat = conn.prepare("SELECT * FROM users").unwrap();
        let userdata = dat
            .query_map([], |row| {
                Ok(User {
                    username: row.get(0).unwrap(),
                    password: row.get(1).unwrap(),
                    token: row.get(2).unwrap(),
                    cores: row.get(3).unwrap()
                })
            })
            .unwrap();

        let users: Result<Vec<User>, _> = userdata.collect();
        let default = vec![User {
            username: "error reading database".to_string(),
            password: "error reading database".to_string(),
            token: "error reading database".to_string(),
            cores:"error reading database".to_string()
        }];

        return users.unwrap_or(default);
    }
    pub fn writedata(new_user: &User) -> Result<usize>{
        let tok = new_user.token.clone();
        Instance::createfolder(format!("minecraftdata/{tok}"),true);
        let conn = Connection::open("data.db").unwrap();
        conn.execute(
            "INSERT INTO users (username, password, token, cores) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![new_user.username, new_user.password, new_user.token,new_user.cores]
        )


    }
}
