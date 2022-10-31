use std::error::Error;

mod sum;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Result: {:?}", sum::exec(2, 5));
    Ok(())
}
