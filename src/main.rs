use std::io::{self, Write};

mod parser;

fn main() {
    let mut text = String::new();

    print!("Text: ");
    io::stdout().flush().expect("how could this fail?");

    io::stdin()
        .read_line(&mut text)
        .expect("should have read the line dude.");

    let wat = parser::parens(&text);

    match wat {
        Ok((a, b)) => {
            println!("first? '{a}'");
            println!("second? '{b}'");
        }
        Err(e) => {
            println!("nope. {e}");
        }
    }
}
