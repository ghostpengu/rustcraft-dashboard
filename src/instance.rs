use std::process::Command;
use std::{ default, fs };
use std::fs::File;
use std::io;



pub struct Instance {}
fn delete_dir_contents(dir_path: &String) -> io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            delete_dir(&path.to_string_lossy().to_string())?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}

fn delete_dir(dir_path: &String) -> io::Result<()> {
    delete_dir_contents(dir_path)?;
    fs::remove_dir(dir_path)?;
    Ok(())
}
impl Instance {
    pub fn writefile(path:String,content:&String){
        match fs::write(path, content) {
            Ok(()) => {
                // If successful, print a success message
                println!("Text successfully written to file.");
            }
            Err(err) => {
                // If an error occurs, print the error message
                eprintln!("Error writing to file: {}", err);
            }
        }
    }
    pub fn readfile(path:String ) -> String{
        match fs::read_to_string(path) {
            Ok(contents) => {
                // If successful, print the contents of the file
                return contents;
            }
            Err(err) => {
                // If an error occurs, print the error message
                return format!("{err}");
            }
        }
    }
    pub fn unzip(name: &String) {
        let file = File::open("minecraftdata/server.zip").unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        archive.extract(format!("{name}")).unwrap();
    }
    pub fn deletefolder(name: String) {
        match delete_dir(&name.to_string()) {
            Ok(_) => println!("Folder deleted successfully"),
            Err(e) => println!("Error deleting folder: {:?}", e),
        }
    }
    pub fn createfolder(name: String, createuser: bool) {
        if createuser {
            Instance::unzip(&name);
        }

        match fs::create_dir_all(name) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    }
    pub fn isrunning(id: &String) -> bool {
        let output = Command::new("tmux")
            .arg("has-session")
            .arg("-t")
            .arg(id)
            .output()
            .expect("Failed to execute tmux command");

        if output.status.success() {
            println!("tmux session 'rust' is running");
            true
        } else {
            println!("tmux session 'rust' is not running");
            false
        }
    }
    pub fn start(token: &String) {
        let out = Command::new("tmux").arg("has-session").arg("-t").arg(&token).output().unwrap();
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
    pub fn send_command(command: String, token: &String) {
        Command::new("tmux")
            .arg("send-keys")
            .arg("-t")
            .arg(&token)
            .arg(command)
            .arg("C-m")
            .spawn()
            .unwrap();
    }
    pub fn destroy_instance(token: &String) {
        Command::new("tmux").arg("kill-session").arg("-t").arg(&token).spawn().unwrap();
    }
    pub fn read_terminal(token: &String) -> String {
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
