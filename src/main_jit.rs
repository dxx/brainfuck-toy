use brainfuck_toy::jit::Interpreter;
use brainfuck_toy::parse_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = parse_data()?;
    let mut interpreter = Interpreter::default();
    interpreter.run(data)?;

    Ok(())
}
