extern crate nix;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

fn main() {

    let handler = thread::spawn(move || {
        let mut openvpn = Command::new("ls")
            .arg("-1")
            .arg("-a")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Could not spawn command 'ls'");

        
        if let Some(ref mut stdout) = openvpn.stdout {
            let reader = BufReader::new(stdout);
    
            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| println!("OpenVPN:{}", line));
        }
        

        let status = openvpn.wait();
        println!("Status: {:?}", status);

    });
    
    handler.join().expect("Thread Panicked");

    println!("Hello, world!");
}

// Buffered Reader for stdout https://github.com/rust-lang-nursery/rust-cookbook/pull/373/files