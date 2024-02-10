use std::{thread, time::Duration };
mod instance;

use instance::*;
mod database;

use database::*;

#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{ context, Template };


static mut USER_DATA: Option<Vec<database::User>> = None;



// Function to read user data into an array
fn load_user_data() {
    unsafe {
        USER_DATA = Some(Database::readdatabase());
    }
}

fn tokenckeck(users: &[User], token: &String) -> bool {
    let _i = User {
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
    if tokenckeck(l, &token) && Instance::isrunning(&token) {
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
        thread::sleep(Duration::from_millis(100));
        Instance::send_command("cd ~/failed/rustcraft-dashboard/target/debug/server".to_string(), &token);
        thread::sleep(Duration::from_micros(500));
        Instance::send_command("./start.sh".to_string(), &token);
        "Started!!!!"
    } else {
       // println!("No Dad?");
        "Check if you are logined if yes click init to start server"
    }
}
#[get("/start/<token>")]
fn startinstance(token: String) -> String {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
  
    if tokenckeck(l, &token) {
        //Instance::writedata(new_user);
        Instance::send_command("cd ~/failed/rustcraft-dashboard/target/debug/server".to_string(), &token);
        thread::sleep(Duration::from_micros(100));
        Instance::send_command("./start.sh".to_string(), &token);
       
    }

    let out = Instance::read_terminal(&token);
    out
}
#[get("/read/<token>")]
fn readinstance(token: String) -> String {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if tokenckeck(l, &token) && Instance::isrunning(&token) {
        let out = Instance::read_terminal(&token);
        println!("{}", out);
        out
    } else {
        "Check if you are logined if yes click init to start server".to_string()
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
