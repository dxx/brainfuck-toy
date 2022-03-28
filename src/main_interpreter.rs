use std::io::prelude::*;
use brainfuck_toy::interpreter::Interpreter;

fn parse_data() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() >= 2);

    let mut f = std::fs::File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    f.read_to_end(&mut data)?;

    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = parse_data()?;
    let mut interpreter = Interpreter::default();
    interpreter.run(data)?;

    Ok(())
}
