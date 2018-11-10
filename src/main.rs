use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: <program> <query> <filename>");
        return;
    }
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {} in file {}", query, filename);
}
