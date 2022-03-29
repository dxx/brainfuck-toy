use super::opcode;

use std::collections;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    SHL,
    SHR,
    ADD,
    SUB,
    LSB,
    RSB,
    GETCHAR,
    PUTCHAR,
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            opcode::OPCODE_SHL => Opcode::SHL,
            opcode::OPCODE_SHR => Opcode::SHR,
            opcode::OPCODE_ADD => Opcode::ADD,
            opcode::OPCODE_SUB => Opcode::SUB,
            opcode::OPCODE_LSB => Opcode::LSB,
            opcode::OPCODE_RSB => Opcode::RSB,
            opcode::OPCODE_GETCHAR => Opcode::GETCHAR,
            opcode::OPCODE_PUTCHAR => Opcode::PUTCHAR,
            _ => panic!("Unsupported opcode {}", u),
        }
    }
}

struct Code {
    opcodes: Vec<Opcode>,
    jump_table: collections::HashMap<usize, usize>,
}

impl Code {
    fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let opcodes: Vec<Opcode> = data.iter()
            .filter(|x| opcode::OPCODE_DICT.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();

        let mut jump_table = collections::HashMap::new();
        let mut stack = Vec::new();
        
        for (i, v) in opcodes.iter().enumerate() {
            if Opcode::LSB == *v {
                stack.push(i);
            }
            if Opcode::RSB == *v {
                let j = stack.pop().ok_or("Pop from empty stack")?;
                // 保存 [ 和 ] 程序计数器的对应关系
                jump_table.insert(j, i);
                jump_table.insert(i, j);
            }
        }
 
        Ok(Code { opcodes, jump_table })
    }

}

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
        let code = Code::from(data)?;
        let opcodes = code.opcodes;
        let opcode_len = opcodes.len();
        let jump_table = code.jump_table;
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
