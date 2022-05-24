use rust_port_sniffer::Arguments;
use std::env;
fn main() {
    let idk = rust_port_sniffer::new(env::args());
    println!("Yes {:?}", idk)
}
