extern crate nix;
extern crate ctrlc;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{thread, time};

fn main() {
    let handler = thread::spawn(|| {
        let mut openvpn = Command::new("sudo")
            .arg("openvpn")
            .arg("--config")
            .arg("pia/CAToronto.ovpn")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Could not spawn command 'openvpn'");

        // Handle reading stdout of openvpn
        if let Some(ref mut stdout) = openvpn.stdout {
            let reader = BufReader::new(stdout);

            let lines = reader.lines().map(|line| line.unwrap());

            for line in lines {
                println!("[OpenVPN]: {}", line);
                if line.contains("Initialization Sequence Completed") {
                    // Check if the TUN0 has been activated
                    println!("TUN ACTIVATED, LAUNCH TRANSMISSION");

                    let addrs = nix::ifaddrs::getifaddrs().unwrap();
                    let mut openvpn_addrs = String::new();

                    for ifaddr in addrs {
                        // Iterate through addresses
                        match ifaddr.address {
                            Some(address) => {
                                if ifaddr.interface_name.eq("tun0") {
                                    println!(
                                        "interface {} address {}",
                                        ifaddr.interface_name, address
                                    );
                                    openvpn_addrs = address.to_str();
                                    break;
                                }
                            }

                            None => {
                                println!(
                                    "interface {} with usupported address family",
                                    ifaddr.interface_name
                                );
                            }
                        }
                    }

                    if !openvpn_addrs.is_empty() {
                        println!("Got IP Address, start Transmission daemon");

                        // Read transmission settigns json and modify bind ip to OpenVPN address
                        // TODO: Implement

                        // Start Transmission-Daemon
                        let mut transmission = Command::new("transmission-daemon")
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .spawn()
                            .expect("Could not spawn command 'transmission-daemon'");

                        if let Some(ref mut stdout) = transmission.stdout {
                            let transmission_reader = BufReader::new(stdout);
                            let transmission_lines = transmission_reader.lines().map(|line| line.unwrap());

                            transmission_lines.for_each(|line| println!("[Transmission]: {}", line));
                        }

                        let transmission_status = transmission.wait();
                        println!("Transmission Status: {:?}", transmission_status);                        

                    } else {
                        println!("Could not start OpenVPN properly")
                    }
                }
            } // End line reading for-loop
        }

        let status = openvpn.wait();
        println!("Status: {:?}", status);
    });

    // Start another handler for signal interrupts, gracefully kill transmission and openvpn
    // TODO: implement

    handler.join().expect("Thread Panicked");

    println!("Hello, world!");
}

// Buffered Reader for stdout https://github.com/rust-lang-nursery/rust-cookbook/pull/373/files
