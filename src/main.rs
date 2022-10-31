use std::error::Error;

mod add;
mod branch;
mod sub;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Result: {:?}", add::exec(2, 5));
    println!("Result: {:?}", sub::exec(10, 3));
    println!("Result: {:?}", branch::exec(10));
    Ok(())
}
