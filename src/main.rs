extern crate nix;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::{thread, time};

fn main() {

    let handler = thread::spawn(|| {
        let mut openvpn = Command::new("openvpn")
            .arg("--config")
            .arg("pia/CAToronto.ovpn")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Could not spawn command 'openvpn'");

        
        if let Some(ref mut stdout) = openvpn.stdout {
            let reader = BufReader::new(stdout);
    
            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| println!("[OpenVPN] {}", line));
        }

        let addrs = nix::ifaddrs::getifaddrs().unwrap();

        for ifaddr in addrs {
            match ifaddr.address {
                Some(address) => {
                    println!("interface {} address {}", ifaddr.interface_name, address);
                },

                None => {
                    println!("interface {} with usupported address family",
                        ifaddr.interface_name);
                }
            }
        }
        

        let status = openvpn.wait();
        println!("Status: {:?}", status);

    });
    
    handler.join().expect("Thread Panicked");

    println!("Hello, world!");
}

// Buffered Reader for stdout https://github.com/rust-lang-nursery/rust-cookbook/pull/373/files