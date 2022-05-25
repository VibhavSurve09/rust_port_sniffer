use std::env;
use std::fmt::Debug;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
const MAX: u16 = 1000;
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
        let flags = args.next().unwrap();
        // args.next() returns an Option type and to extract value from it we use unwrap method and it returns String type
        if let Ok(ip) = IpAddr::from_str(args.next().unwrap().as_str()) {
            return Ok(Arguments {
                flag: String::from(flags),
                ip_address: ip,
                thread: 10000,
            });
        };
        Err("Invalid IP Address")
    }

    fn scan(tx: Sender<u16>, start_port: u16, ip: IpAddr, num_threads: u16) {
        let mut port: u16 = start_port + 1;

        if TcpStream::connect_timeout(&SocketAddr::new(ip, port), Duration::from_secs(1)).is_ok() {
            println!(".");
            //Not completely understood what flush does
            //https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();

            port += num_threads;
        }
    }
}

pub fn run(args: env::Args) {
    let arguments = Arguments::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
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
    open_ports.sort_unstable();
    for port in open_ports {
        println!("Port {} is open", port);
    }
}
