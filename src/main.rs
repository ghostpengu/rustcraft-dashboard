use std::{ io::stdin, process::{ exit, Command }, thread, time::Duration };

use rusqlite::{ params, Connection, Result };
#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{ context, Template };
static mut RUNNING: bool = false;

static mut USER_DATA: Option<Vec<User>> = None;
struct Instance {}

impl Instance {
    fn start(token: &String) {
        let mut out = Command::new("tmux")
            .arg("has-session")
            .arg("-t")
            .arg(&token)
            .output()
            .unwrap();
        let stdout_str = String::from_utf8_lossy(&out.stdout).contains("test");
        if stdout_str == false {
            Command::new("tmux")
                .arg("new-session")
                .arg("-d")
                .arg("-s")
                .arg(&token)
                .spawn()
                .unwrap();
        }
    }
    fn send_command(command: String, token: &String) {
        Command::new("tmux")
            .arg("send-keys")
            .arg("-t")
            .arg(&token)
            .arg(command)
            .arg("C-m")
            .spawn()
            .unwrap();
    }
    fn destroy_instance(token: &String) {
        Command::new("tmux").arg("kill-session").arg("-t").arg(&token).spawn().unwrap();

        unsafe {
            RUNNING = false;
        }
    }
    fn read_terminal(token: &String) -> String {
        let out = Command::new("tmux")
            .arg("capture-pane")
            .arg("-p")
            .arg("-t")
            .arg(token)
            .output()
            .unwrap();
        let stdout_str = String::from_utf8_lossy(&out.stdout);
        stdout_str.trim().to_string()
    }
}

struct Database;

impl Database {
    fn dataread() {
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

    fn readdatabase() -> Vec<User> {
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

        return users.unwrap_or_default();
    }
    fn writedata(new_user: &User) {
        let conn = Connection::open("data.db").unwrap();
        conn.execute(
            "INSERT INTO users (username, password, token) VALUES (?1, ?2, ?3)",
            rusqlite::params![new_user.username, new_user.password, new_user.token]
        ).unwrap_or(0);
    }
}

// Function to read user data into an array
fn load_user_data() {
    unsafe {
        USER_DATA = Some(Database::readdatabase());
    }
}

fn tokenckeck(users: &[User], token: &String) -> bool {
    let i = User {
        username: String::from("unknow"),
        password: String::from("f"),
        token: String::from("F"),
    };

    for user in users {
        if user.matchtoken(token) {
            return true;
        }
    }

    return false;
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    token: String,
}

impl User {
    fn new(username: &str, password: &str, token: &str) -> Self {
        User {
            username: String::from(username),
            password: String::from(password),
            token: String::from(token),
        }
    }

    fn matchtoken(&self, tok: &String) -> bool {
        self.token == *tok
    }
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

fn generate_token(pass: String) -> u64 {
    let hash = fnv1a(pass);
    println!("Token: {}", hash);
    return hash;
}

#[get("/")]
fn homepage() -> Template {
    Template::render("index", context! {})
}

#[get("/register")]
fn regipage() -> Template {
    Template::render("regi", context! {})
}

#[get("/c/<cmd>/<token>")]
fn sendcommand(cmd: String, token: String) {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if tokenckeck(l, &token) {
        Instance::send_command(cmd, &token);
    } else {
        println!("no dad?")
    }
}

#[get("/createuser/<user>/<pass>")]
fn createuser(user: String, pass: String) -> String {
    let clone = pass.clone();
    let u = User {
        username: user,
        password: pass,
        token: generate_token(clone).to_string(),
    };

    Database::writedata(&u);

    format!("{}", &u.token)
}

#[get("/init/<token>")]
fn createinstance(token: String) -> &'static str {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if tokenckeck(l, &token) {
        Instance::start(&token);
        println!("started instance");
        "Started!!!!"
    } else {
        println!("No Dad?");
        "Sorry please rigister"
    }
}
#[get("/start/<token>")]
fn startinstance(token: String) -> String {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if (unsafe { RUNNING }) {
    } else if tokenckeck(l, &token) {
        let new_user = User {
            username: "pingu".to_string(),
            password: "Simple".to_string(),
            token: "WOW".to_string(),
        };
        //Instance::writedata(new_user);
        Instance::send_command("cd ~/minecraft-dashboard/target/debug/server".to_string(), &token);
        thread::sleep(Duration::from_micros(100));
        Instance::send_command("./start.sh".to_string(), &token);
        unsafe {
            RUNNING = true;
        }
    }

    let out = Instance::read_terminal(&token);
    out
}
#[get("/read/<token>")]
fn readinstance(token: String) -> String {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if tokenckeck(l, &token) {
        let out = Instance::read_terminal(&token);
        println!("{}", out);
        out
    } else {
        "Please Register".to_string()
    }
}

#[get("/exit/<token>")]
fn exitinstance(token: String) -> String {
    let out = Instance::read_terminal(&token);
    println!("{}", out);
    Instance::destroy_instance(&token);
    out
}
#[launch]
fn rocket() -> _ {
    thread::spawn(move || {
        load_user_data();
        unsafe {
            let l = &USER_DATA;
            println!("{:?}", l)
        }
    });

    rocket
        ::build()
        .mount(
            "/",
            routes![
                createinstance,
                readinstance,
                startinstance,
                exitinstance,
                homepage,
                sendcommand,
                regipage,
                createuser
            ]
        )
        .attach(Template::fairing())
}
