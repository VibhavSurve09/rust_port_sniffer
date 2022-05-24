use std::env;
use std::fmt::Debug;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
pub struct Arguments {
    pub flags: String,
    pub ip_address: IpAddr,
    pub thread: u16,
}
impl Arguments {
    pub fn new(mut args: env::Args) -> Result<Arguments, &'static str> {
        //args.next() is to skip the target path
        args.next();
        let flags = args.next();
        if let Ok(ip) = IpAddr::from_str(args.next().unwrap().as_str()) {
            return Ok(Arguments {
                flags: String::from(""),
                ip_address: ip,
                thread: 4,
            });
        };
        Err("Oops")
    }
}
