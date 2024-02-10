use std::{ process::{ Command }, thread, time::Duration };
pub struct Instance {}

impl Instance {
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
        let out = Command::new("tmux")
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
