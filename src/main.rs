use nix::pty::forkpty;
use nix::unistd::ForkResult;
use std::os::fd::RawFd;
use std::process::Command;

fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
    unimplemented!()
}

fn spawn_pty_with_shell(default_shell: String) -> RawFd {
    match forkpty(None, None) {
        Ok(fork_pty_res) => {
            let stdout_fd = fork_pty_res.master;
            if let ForkResult::Child == fork_pty_res.fork_result {
                // now executing in the child process
                Command::new()
            }
        }
    }
}

fn main() {
    let default_shell = std::env::var("SHELL").unwrap_or("/bin/sh".to_string());
    let stdout_fd = spawn_pty_with_shell(default_shell);
    let mut read_buffer = vec![];

    loop {
        match read_from_fd(stdout_fd) {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            }
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(0);
            }
        }
    }
}
