use std::error::Error;
use std::fs;

pub struct  Config{
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{

        // Improve error message
        if args.len() < 3{
            return Err("nor enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }
}

 //extract logic from main
//return errors from the run fn
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("with text\n{}", contents);

    Ok(())
}