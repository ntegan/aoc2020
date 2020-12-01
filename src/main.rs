mod input;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let input = input::read_until_eof()?;
    println!("{}", input);

    Ok(())
}
