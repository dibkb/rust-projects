use std::env;
use std::net::{IpAddr};
use std::str::FromStr;

struct Arguments {
    flag: String,
    ip_address: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: Vec<String>) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Too few arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone();
        match IpAddr::from_str(&f){
            Ok(ipaddr) => {

                return Ok(Arguments {
                    flag: String::from(""),
                    ip_address: ipaddr,
                    threads: 4,
                });
            }
            Err(_) => {

                if (f.contains("-h") || f.contains("-help")) && args.len() == 2{
                    println!(
                        "Usage: -j to select how many threads you want
                    \n\r       -h or -help to show this help message"
                    );
                    return Err("help");
                }else if f.contains("-h") || f.contains("-help"){
                    return Err("too many arguments");
                }else if f.contains("-j"){
                    let ipaddr =  match IpAddr::from_str(&args[3]){
                        Ok(s)=>s,
                        Err(_)=>  return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                    };
                    
                    let threads =  match args[2].parse::<u16>(){
                        Ok(s)=> s,
                        Err(_)=> return Err("failed to parse thread number"),
                    };

                    return Ok(Arguments {
                        flag : f,
                        threads,
                        ip_address: ipaddr
                    })
                }
                else{
                    return Err("invalid syntax")
                }

            }
        };
        
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match Arguments::new(args) {
        Ok(args) => {
            // TODO: Implement main logic
            println!("Arguments parsed successfully");
            println!("flag: {}", args.flag);
            println!("threads: {}", args.threads);
            println!("ip: {}", args.ip_address);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
