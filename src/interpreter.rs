use super::{opcode, opcode::Opcode};

use std::io::prelude::*;

/// 解释器
pub struct Interpreter {
    stack: Vec<u8>, // 保存解释执行的结果
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {

    pub fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = opcode::Code::from(data)?;
        let opcodes = code.opcodes();
        let opcode_len = opcodes.len();
        let jump_table = code.jump_table();
        let mut pc = 0; // 程序计数器
        let mut s_pointer = 0; // 指针
        
        loop {
            if pc >= opcode_len {
                break;
            }

            let opcode = &opcodes[pc];
            match opcode {
                Opcode::SHL => {
                    if s_pointer > 0 {
                        s_pointer -= 1;
                    }
                }
                Opcode::SHR => {
                    s_pointer += 1;
                    if s_pointer == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                Opcode::ADD => {
                    self.stack[s_pointer] = self.stack[s_pointer].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[s_pointer] = self.stack[s_pointer].overflowing_sub(1).0;
                }
                Opcode::LSB => {
                    if self.stack[s_pointer] == 0 {
                        pc = jump_table[&pc];
                    }
                }
                Opcode::RSB => {
                    if self.stack[s_pointer] != 0 {
                        pc = jump_table[&pc];
                    }
                }
                Opcode::GETCHAR => {
                    let mut buf = [0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[s_pointer] = buf[0];
                }
                Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[s_pointer]])?;
                }
            }

            pc += 1;
        }

        Ok(())
    }
}
