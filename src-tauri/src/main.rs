// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::env;

fn main() {
    #[cfg(target_os = "macos")]
    if unsafe { libc::geteuid() } != 0 {
        let exe = env::current_exe().unwrap();
        let exe_path = exe.to_str().unwrap();

        // In release mode (bundled app), we want to fire-and-forget so the non-root process exits
        // and the new root process takes over.
        // In debug mode, we block so the terminal session stays alive.
        let prompt = "BeProx needs administrative privileges to bind port 80 and modify /etc/hosts.";
        let script = if cfg!(debug_assertions) {
            format!("do shell script \"'{exe_path}'\" with administrator privileges with prompt \"{prompt}\"")
        } else {
            format!("do shell script \"'{exe_path}' &> /dev/null &\" with administrator privileges with prompt \"{prompt}\"")
        };

        match Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output() 
        {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("Elevation failed or cancelled: {}", String::from_utf8_lossy(&output.stderr));
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute osascript: {}", e);
                std::process::exit(1);
            }
        }
        
        // If we are here, osascript succeeded.
        // In release mode, the new process is backgrounded, so we should exit.
        // In debug mode, osascript blocked until the child exited, so we exit now too.
        return;
    }

    app_lib::run()
}
