use std::{ thread, time::Duration };
mod console;
use console::*;
mod instance;
use instance::*;
mod database;
use rocket::serde::Serialize;
use database::*;
use rocket::serde::json::Json;
use rocket::fs::FileServer;
use core::ptr::addr_of;
use std::env;
extern crate num_cpus;
use serde::{ Deserialize };
use std::collections::HashMap;
#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{ context, Template };

static mut USER_DATA: Option<Vec<database::User>> = None;


#[derive(Serialize, Debug)]
struct Client {
    username: String,
    token: String,
}
// Function to read user data into an array
fn load_user_data() {
    unsafe {
        USER_DATA = Some(Database::readdatabase());
    }
}

#[get("/getproperties/<token>")]
fn get_properties(token: String) -> String {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if tokenckeck(l, &token) {
        let out = Instance::readfile(format!("minecraftdata/{token}/server.properties"));
        return out;
    }

    "failed login please".to_string()
}
#[derive(Debug, Deserialize)]
struct Properties {
    out: String,
}

#[post("/setproperties/<token>", data = "<content>")]
fn set_properties(token: String, content: Json<Properties>) -> String {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if tokenckeck(l, &token) {
        let out = &content.out;

        Instance::writefile(format!("minecraftdata/{token}/server.properties"), out);
        return "Wokrs".to_string();
    }

    return "Wokrs".to_string();
}
#[get("/person")]
fn get_user() -> Json<User> {
    let person = User {
        username: "error reading database".to_string(),
        password: "error reading database".to_string(),
        token: "error reading database".to_string(),
        cores:"error reading database".to_string()
    };
    Json(person)
}

fn logincheck(users: &[User], username: &String, pass: &String) -> bool {
    for user in users {
        if user.login(username, pass) {
            return true;
        }
    }
    return false;
}

fn tokenckeck(users: &[User], token: &String) -> bool {
    let _i = User {
        username: String::from("unknow"),
        password: String::from("f"),
        token: String::from("F"),
        cores:String::from("0,1")
    };

    for user in users {
        if user.matchtoken(token) {
            return true;
        }
    }

    return false;
}
fn getuser(users: &[User], token: &String) -> User {
    let _i = User {
        username: String::from("unknow"),
        password: String::from("f"),
        token: String::from("F"),
        cores:String::from("0,1")
    };

    for user in users {
        if user.matchtoken(token) {
            let i = user.clone();
            return i;
        }
    }

    return _i;
}
#[get("/")]
fn homepage() -> Template {
    Template::render("index", context! {})
}
#[get("/properties")]
fn properties() -> Template {
    Template::render("properties", context! {})
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
fn createuser(user: String, pass: String) -> Json<Client> {
    let passclone = pass.clone();
    let usernameclone = user.clone();
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    let u = User {
        username: user,
        password: pass,
        token: generate_token(usernameclone, passclone).to_string(),
        cores:String::from("0,1")
    };
    if !logincheck(l, &u.username, &u.password) {
        match Database::writedata(&u) {
            Ok(suc) => println!("{suc}"),
            Err(err) => println!("{err}"),
        }

        reloaddata();
    }

    let client = Client {
        username: u.username,
        token: u.token,
    };

    Json(client)
}

#[get("/init/<token>")]
fn createinstance(token: String) -> &'static str {
    let l = unsafe { &USER_DATA.as_ref().unwrap() };
    if tokenckeck(l, &token) {
        Instance::start(&token);
        let path = env::current_dir().unwrap();
        let x = path.display();
        println!("started instance");
        thread::sleep(Duration::from_millis(100));
        let usr = getuser(l,&token);
        let cores = usr.cores;
        Instance::send_command(format!("cd {x}/minecraftdata/{token}"), &token);
        thread::sleep(Duration::from_millis(100));
        Instance::send_command(format!("chmod 777 run.sh"), &token);
        thread::sleep(Duration::from_micros(500));
        Instance::send_command(format!("./run.sh {cores}"), &token);
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
        let path = env::current_dir().unwrap();
        let x = path.display();
        let usr = getuser(l,&token);
        let cores = usr.cores;
        Instance::send_command(format!("cd {x}/minecraftdata/{token}"), &token);
        thread::sleep(Duration::from_micros(100));
        Instance::send_command(format!("./run.sh {cores}"), &token);
        //java -Xmx1024M -Xms1024M -jar server.jar nogui
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
fn reloaddata() {
    thread::spawn(move || {
        load_user_data();
        unsafe {
            let l = addr_of!(USER_DATA);
            println!("{:?}", l)
        }
    });
}

#[launch]
fn rocket() -> _ {
    console();
    reloaddata();
    Instance::createfolder("minecraftdata".to_string(), false);
    rocket
        ::build()
        .mount(
            "/",
            routes![
                properties,
                set_properties,
                get_properties,
                createinstance,
                readinstance,
                startinstance,
                exitinstance,
                homepage,
                sendcommand,
                regipage,
                createuser,
                get_user
            ]
        )
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
