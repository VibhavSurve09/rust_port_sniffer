use std::env;
use std::fmt::Debug;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

// Total number of ports in system
const MAX: u16 = 65535;
#[derive(Debug)]
pub struct Arguments {
    pub flag: String,
    pub ip_address: IpAddr,
    pub thread: u16,
}

impl Arguments {
    fn new(mut args: env::Args) -> Result<Arguments, &'static str> {
        //Firtst args.next() is to skip the target path
        args.next();
        let flags = args.next();
        match flags {
            Some(flag) => {
                if flag.trim().contains("-t") {
                    let threads_or_ip = args.next();
                    let threads_or_ip_clone = threads_or_ip.clone();
                    match threads_or_ip.unwrap().parse::<u16>() {
                        Ok(num_threads) => {
                            // args.next() returns an Option type and to extract value from it we use unwrap method and it returns String type
                            if let Ok(ip) = IpAddr::from_str(args.next().unwrap().as_str()) {
                                return Ok(Arguments {
                                    flag: String::from(flag),
                                    ip_address: ip,
                                    thread: num_threads,
                                });
                            };
                        }
                        _ => {
                            if let Ok(ip) = IpAddr::from_str(threads_or_ip_clone.unwrap().as_str())
                            {
                                return Ok(Arguments {
                                    flag: String::from(flag),
                                    ip_address: ip,
                                    thread: 100,
                                });
                            };
                        }
                    }
                } else {
                    return Err("Use -t flag for threads");
                }
            }
            _ => {
                args.next();
                if let Ok(ip) = IpAddr::from_str(args.next().unwrap().as_str()) {
                    return Ok(Arguments {
                        flag: String::from(""),
                        ip_address: ip,
                        thread: 100,
                    });
                };
            }
        }
        Err("Something went wrong!!")
    }

    fn scan(tx: Sender<u16>, start_port: u16, ip: IpAddr, num_threads: u16) {
        let mut port: u16 = start_port + 1;
        while (MAX - port) >= num_threads {
            if TcpStream::connect_timeout(&SocketAddr::new(ip, port), Duration::from_secs(1))
                .is_ok()
            {
                print!(".");
                //https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            port += num_threads;
        }
    }
}

pub fn run(args: env::Args) {
    let arguments = Arguments::new(args).unwrap_or_else(|err| {
        println!("Invalid arguments {}", err);
        process::exit(1);
    });
    let num_threads = arguments.thread;
    let ip_address = arguments.ip_address;
    let flag = arguments.flag;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || Arguments::scan(tx, i, ip_address, num_threads));
    }

    let mut open_ports: Vec<u16> = Vec::new();
    drop(tx);

    for port in rx {
        open_ports.push(port);
    }
    println!();
    open_ports.sort();
    for port in open_ports {
        println!("Port {} is open", port);
    }
}

// If we have to pass in the flags like '-j' or '-some_flag' it should always precede with '--'
