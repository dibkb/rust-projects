use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Error parsing arguments {}",err);
        process::exit(1);
    });

    let contents = fs::read_to_string(config.filename).expect(
        "filename does not exist"
    );

    println!("{}?",contents);
}


struct Config {
    query : String,
    filename: String
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config,&str>{

        if args.len() < 3{
            println!("Not enough arguments");
        };

        let (query,filename) = match (args.get(1),args.get(2)) {
            (Some(q),Some(f))=> (q,f),
            _ => {
                println!("query and filename should be present");
                process::exit(1);
            }
        };

        Ok(Config { query : query.to_string(), filename : filename.to_string() })
        
    }
}