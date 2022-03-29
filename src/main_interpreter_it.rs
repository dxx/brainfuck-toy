use brainfuck_toy::interpreter_it::Interpreter;
use brainfuck_toy::parse_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = parse_data()?;
    let mut interpreter = Interpreter::default();
    interpreter.run(data)?;

    Ok(())
}
