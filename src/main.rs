use std::error::Error;

mod add;
mod sub;
mod branch;

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = std::env::args().collect();
    match &*argv[1] {
        "add" => println!("Result: {:?}", add::exec(2, 5)),
        "sub" => println!("Result: {:?}", sub::exec(10, 3)),
        "branch" => println!("Result: {:?}", branch::exec(10)),
        _ => println!("Unknown command"),
    }

    Ok(())
}
