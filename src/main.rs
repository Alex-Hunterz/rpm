use nix::sys::signal::{kill, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{fork, ForkResult, Pid};
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let mut tracked_pids: Vec<Pid> = Vec::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "exit" => break,
            
            "help" => {
                println!("Commands: help, create, list, ps, kill <pid>, exit");
            }
            
            "create" => {
                match unsafe { fork() } {
                    Ok(ForkResult::Parent { child }) => {
                        tracked_pids.push(child);
                        println!("Created process with PID {}", child);
                    }
                    Ok(ForkResult::Child) => {
                        // Child process sleeps then exits
                        std::thread::sleep(std::time::Duration::from_secs(30));
                        process::exit(0);
                    }
                    Err(e) => println!("Fork failed: {}", e),
                }
            }
            
            "list" => {
                // Remove dead processes from tracking
                tracked_pids.retain(|pid| fs::metadata(format!("/proc/{}", pid)).is_ok());
                
                if tracked_pids.is_empty() {
                    println!("No tracked processes");
                } else {
                    println!("Tracked processes:");
                    for pid in &tracked_pids {
                        println!("- PID {} [alive]", pid);
                    }
                }
            }
            
            "ps" => {
                println!("All system processes:");
                if let Ok(entries) = fs::read_dir("/proc") {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let name = entry.file_name();
                            let name_str = name.to_string_lossy();
                            
                            // Check if it's a PID directory (all digits)
                            if name_str.chars().all(|c| c.is_ascii_digit()) {
                                let pid = name_str.as_ref();
                                
                                // Try to read the command name
                                let comm_path = format!("/proc/{}/comm", pid);
                                let comm = fs::read_to_string(comm_path)
                                    .unwrap_or_else(|_| "unknown".to_string())
                                    .trim()
                                    .to_string();
                                
                                println!("- PID {} [{}]", pid, comm);
                            }
                        }
                    }
                }
            }
            
            "kill" => {
                if parts.len() != 2 {
                    println!("Usage: kill <pid>");
                    continue;
                }
                
                match parts[1].parse::<i32>() {
                    Ok(pid_num) => {
                        let pid = Pid::from_raw(pid_num);
                        
                        match kill(pid, Signal::SIGKILL) {
                            Ok(_) => {
                                println!("Sent SIGKILL to process {}", pid);
                                
                                // Wait for the child to exit and clean up zombie
                                match waitpid(pid, Some(WaitPidFlag::WNOHANG)) {
                                    Ok(WaitStatus::Exited(_, _)) => {
                                        println!("Process {} killed and cleaned up", pid);
                                    }
                                    Ok(WaitStatus::Signaled(_, _, _)) => {
                                        println!("Process {} killed and cleaned up", pid);
                                    }
                                    Ok(WaitStatus::StillAlive) => {
                                        // Try non-blocking wait
                                        std::thread::sleep(std::time::Duration::from_millis(10));
                                        let _ = waitpid(pid, Some(WaitPidFlag::WNOHANG));
                                        println!("Process {} killed", pid);
                                    }
                                    _ => {
                                        println!("Process {} killed", pid);
                                    }
                                }
                            }
                            Err(e) => println!("Failed to kill process {}: {}", pid, e),
                        }
                    }
                    Err(_) => println!("Invalid PID: {}", parts[1]),
                }
            }
            
            _ => println!("Unknown command: {}", parts[0]),
        }
    }
    
    println!("Goodbye!");
}
