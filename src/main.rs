use clap::Parser;
use std::fs;
use std::path::Path;
use std::io::{self, Read};

#[derive(Parser)]
struct Cli {
    action: String,
    function: String, 
}

fn main() {
    let args = Cli::parse();
    
    match args.action.as_str() {
        "get" => access_file(&args.function).unwrap(),
        "doc" => println!("You want the doc of a function ?"),
        _ => println!("You're action doesn't exist."),
    }
}

fn access_file(filename: &str) -> io::Result<()> {
    let utils_dir = "../ts-utils";
    let filepath = format!("{}/{}{}", utils_dir, filename, ".ts");
    let path = Path::new(&filepath);
    
    if !path.exists() {
        println!("Error: File '{}' does not exist", filepath);
        return Ok(());
    }
    
    // Read and display the file content
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    println!("Contents of {}:", filepath);
    println!("{}", contents);
    
    Ok(())
}