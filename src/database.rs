use rocket::serde::Serialize;
use crate::instance;
use instance::*;
use rusqlite::{ Connection, Result };
pub struct Database;
#[derive(Serialize, Debug,Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub token: String,
}
fn fnv1a<T: AsRef<[u8]>>(data: T) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    for byte in data.as_ref() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub fn generate_token(usename:String,pass: String) -> u64 {
    let hash = fnv1a(format!("{usename}{pass}"));
    return hash;
}
impl User {
    pub fn new(username: &str, password: &str, token: &str) -> Self {
        User {
            username: String::from(username),
            password: String::from(password),
            token: String::from(token),
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
        };
        let conn = Connection::open("data.db").unwrap();
        let mut dat = conn.prepare("SELECT * FROM users").unwrap();
        let userdata = dat
            .query_map([], |row| {
                Ok(User {
                    username: row.get(0).unwrap(),
                    password: row.get(1).unwrap(),
                    token: row.get(2).unwrap(),
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
                })
            })
            .unwrap();

        let users: Result<Vec<User>, _> = userdata.collect();
        let default = vec![User {
            username: "error reading database".to_string(),
            password: "error reading database".to_string(),
            token: "error reading database".to_string(),
        }];

        return users.unwrap_or(default);
    }
    pub fn writedata(new_user: &User) -> Result<usize>{
        let tok = new_user.token.clone();
        Instance::createfolder(format!("minecraftdata/{tok}"),true);
        let conn = Connection::open("data.db").unwrap();
        conn.execute(
            "INSERT INTO users (username, password, token) VALUES (?1, ?2, ?3)",
            rusqlite::params![new_user.username, new_user.password, new_user.token]
        )

    }
}
