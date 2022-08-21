use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    println!("hello {name}");
}
