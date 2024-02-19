use nix::pty::forkpty;
use nix::unistd::read;
use nix::unistd::ForkResult;
use std::os::fd::RawFd;
use std::process::Command;

fn read_from_pty_fd(fd: RawFd) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65536];
    let read_result = read(fd, &mut read_buffer);
    match read_result {
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_e) => None,
    }
}

fn spawn_pty_with_shell(default_shell: String) -> RawFd {
    match forkpty(None, None) {
        Ok(fork_pty_res) => {
            let stdout_fd = fork_pty_res.master;
            if let ForkResult::Child = fork_pty_res.fork_result {
                // now executing in the child process
                Command::new(&default_shell)
                    .spawn()
                    .expect("shell failed to start!");
                std::thread::sleep(std::time::Duration::from_millis(1000));
                std::process::exit(0);
            }
            stdout_fd
        }
        Err(e) => {
            panic!("failed to fork {:?}", e);
        }
    }
}

fn main() {
    let default_shell = std::env::var("SHELL").unwrap_or("/bin/sh".to_string());
    //let default_shell = "/bin/sh".to_string();
    let stdout_fd = spawn_pty_with_shell(default_shell);
    let mut read_buffer = vec![];

    loop {
        match read_from_pty_fd(stdout_fd) {
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
