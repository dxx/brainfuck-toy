#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use brainfuck_toy::jit::Interpreter;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use brainfuck_toy::parse_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        let data = parse_data()?;
        let mut interpreter = Interpreter::default();
        interpreter.run(data)?;
    }

    Ok(())
}
