use std::{process::{Command, exit}, thread, time::Duration, io::stdin};

use rusqlite::{params, Connection, Result};
#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
static mut RUNNING: bool = false;
struct Instance{

}

impl Instance {

    fn start()
    {
        let mut out = Command::new("tmux")
            .arg("has-session")
            .arg("-t")
            .arg("test")
            .output().unwrap();
        let stdout_str = String::from_utf8_lossy(&out.stdout).contains("test");
        if stdout_str == false{
            Command::new("tmux")
            .arg("new-session")
            .arg("-d")
            .arg("-s")
            .arg("test")
            .spawn().unwrap(); 
        }
       
        
    }
    fn send_command(command:String)
    {
       
        Command::new("tmux")
            .arg("send-keys")
            .arg("-t")
            .arg("test")
            .arg(command)
            .arg("C-m")
            .spawn().unwrap(); 
       
        
        
    }
    fn destroy_instance(){
        Command::new("tmux")
        .arg("kill-session")
        .arg("-t")
        .arg("test")

        .spawn().unwrap(); 
        
        unsafe{
            RUNNING = false
        }
    }
    fn read_terminal() -> String{
        let out = Command::new("tmux")
        .arg("capture-pane")
        .arg("-p")
        .arg("-t")
        .arg("test")

        .output().unwrap(); 
        let stdout_str = String::from_utf8_lossy(&out.stdout);
        stdout_str.trim().to_string()
    }
    fn dataread(){
        let conn = Connection::open("data.db").unwrap();
        let mut dat = conn.prepare("SELECT * FROM users").unwrap();
        let userdata = dat.query_map([], |row| {
            Ok(User {
                username: row.get(0).unwrap(),
                password: row.get(1).unwrap(),
                token: row.get(2).unwrap(),
            })
        }).unwrap();
        for username in userdata {
            println!("{}", username.unwrap().username);
        }
    }
    fn writedata(new_user:&User){

        let conn = Connection::open("data.db").unwrap();
        conn.execute(
            "INSERT INTO users (username, password, token) VALUES (?1, ?2, ?3)",
            rusqlite::params![new_user.username, new_user.password, new_user.token],
        ).unwrap_or(0);
    }
}


#[derive(Debug)]
struct User{

    username: String,
    password: String,
    token: String,

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

fn generate_token(pass:String) -> u64{
    let hash = fnv1a(pass);
    println!("Token: {}",hash);
    return hash;
}

#[get("/")]
fn homepage() -> Template {
    Template::render("index",context! {})
}

#[get("/register")]
fn regipage() -> Template {
    Template::render("regi",context! {})
}


#[get("/c/<cmd>")]
fn sendcommand(cmd:String){
    Instance::send_command(cmd);
    
}





#[get("/createuser/<user>/<pass>")]
fn createuser(user:String,pass:String) -> String {
    let clone = pass.clone();
    let u = User{
        username: user,
        password: pass,
        token: generate_token(clone).to_string()

    };
    
    Instance::writedata(&u);
   
    format!("{:?}",&u)
}

#[get("/init")]
fn createinstance() -> &'static str {
    Instance::start();
    println!("started instance");
    "Started!!!!"
}
#[get("/start")]
fn startinstance() -> String {
    if unsafe {RUNNING}{

    }
    else {
        let new_user =User{
            username: "pingu".to_string(),
            password:"Simple".to_string(),
            token:"WOW".to_string(),
        }; 
        //Instance::writedata(new_user);
        Instance::send_command("cd ~/minecraft-dashboard/target/debug/server".to_string());
        thread::sleep(Duration::from_micros(100));
        Instance::send_command("./start.sh".to_string());  
        unsafe{
            RUNNING = true
        }
    }
   
    let out = Instance::read_terminal();
    out
}
#[get("/read")]
fn readinstance() -> String {
    Instance::dataread();
    
    let out = Instance::read_terminal();
    println!("{}",out);
    out
}


#[get("/exit")]
fn exitinstance() -> String {
    let out = Instance::read_terminal();
    println!("{}",out);
    Instance::destroy_instance();
    out
}
#[launch]
fn rocket() -> _ {
   
    rocket::build().mount("/", routes![createinstance,readinstance,startinstance,exitinstance,homepage,sendcommand,regipage,createuser]).attach(Template::fairing())
}

