use std::error::Error;

mod add;
mod sub;

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = std::env::args().collect();
    match &*argv[1] {
        "add" => println!("Result: {:?}", add::exec(2, 5)),
        "sub" => println!("Result: {:?}", sub::exec(10, 3)),
        _ => println!("Unknown command"),
    }

    Ok(())
}
