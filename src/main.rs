use std::env;
use std::fs;
use std::process;

// Steps in building a minigrep command line application
// Accepting command line arguments
// Reading a file
// Error handling

fn main() {

    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments {}", err);

        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);


}

fn run(config: Config){
    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("with text\n{}", contents);
}

struct  Config{
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str>{

        // Improve error message
        if args.len() < 3{
            return Err("nor enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }
}




