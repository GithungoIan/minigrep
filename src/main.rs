use std::env;
use std::fs;

// Steps in building a minigrep command line application
// Accepting command line arguments
// Reading a file
fn main() {

    let args:Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("with text\n{}", contents);
}
