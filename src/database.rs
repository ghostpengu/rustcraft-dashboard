use rocket::serde::Serialize;

use rusqlite::{ Connection, Result };
pub struct Database;
#[derive(Serialize,Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub token: String,
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
}

impl Database {
    pub fn deletedata() -> Result<()> {
        let conn = Connection::open("data.db").unwrap();

        conn.execute("DELETE FROM users", [])?;
        Ok(())
    }
    pub fn dataread() {
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

        for username in userdata {
            println!("{}", username.unwrap().username);
        }
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
    pub fn writedata(new_user: &User) {
        let conn = Connection::open("data.db").unwrap();
        conn.execute(
            "INSERT INTO users (username, password, token) VALUES (?1, ?2, ?3)",
            rusqlite::params![new_user.username, new_user.password, new_user.token]
        ).unwrap_or(0);
    }
}
