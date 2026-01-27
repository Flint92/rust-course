use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));
    let file = std::fs::File::open(std::env::current_exe()?)?;
    println!("lines in file: {}", count_lines(file));

    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {buffer:?}");

    Ok(())
}

fn count_lines<R: Read>(reader: R) -> usize{
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<(), Box<dyn Error>> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())?;

    Ok(())
}
