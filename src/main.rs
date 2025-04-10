use clap::Parser;

#[derive(Parser)]
struct Cli {
    action: String,
    function: String, 
}

fn main() {
    let args = Cli::parse();
    
    match args.action.as_str() {
        "get" => println!("You want to get a function ?"),
        "doc" => println!("You want the doc of a function ?"),
        _ => println!("You're action doesn't exist."),
    }
}
