pub fn read_until_eof() -> Result<String, Box<dyn std::error::Error>> {
    use std::fmt::Write;
    use std::io;
    let mut input = String::new();
    let buf = String::new();

    loop {
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            break;
        }
    }

    Ok(input)
}
