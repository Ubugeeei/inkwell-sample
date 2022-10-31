use std::error::Error;

mod add;

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = std::env::args().collect();
    if argv[1] == "add" {
        println!("Result: {:?}", add::exec(2, 5));
    }
    Ok(())
}
