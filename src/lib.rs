pub mod opcode;
pub mod interpreter;
pub mod interpreter_it;
pub mod jit;

#[cfg(all(target_arch = "aarch64"))]
pub mod jit_aarch64;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub mod jit_x64;

#[cfg(all(target_arch = "aarch64"))]
pub use jit_aarch64::*;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use jit_x64::*;

use std::io::prelude::*;

pub fn parse_data() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() >= 2);

    let mut f = std::fs::File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    f.read_to_end(&mut data)?;

    Ok(data)
}
