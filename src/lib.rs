pub mod opcode;
pub mod interpreter;
pub mod interpreter_it;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub mod jit;

use std::io::prelude::*;

pub fn parse_data() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() >= 2);

    let mut f = std::fs::File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    f.read_to_end(&mut data)?;

    Ok(data)
}
