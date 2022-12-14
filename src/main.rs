// example:$ cargo run searchstring example-filename.txt
use std::env;
use std::fs;
use std::process;

fn main() {
    //collect arguments from command line
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    //create Config struct with parsed arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //printing the variables to console for development
    println!("Searching for => {}", config.query);
    println!("In file => {}", config.filename);

    //read file, save text to var, and print to console
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong with reading the file");
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
     Ok(Config { query, filename})
    }
}
