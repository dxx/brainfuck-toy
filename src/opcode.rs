use std::collections;
use lazy_static::lazy_static;

const OPCODE_SHL: u8     = 0x3c; // <: 指针减一
const OPCODE_SHR: u8     = 0x3e; // >: 指针加一
const OPCODE_ADD: u8     = 0x2b; // +: 指针指向的字节的值加一
const OPCODE_SUB: u8     = 0x2d; // -: 指针指向的字节的值减一
const OPCODE_LSB: u8     = 0x5b; // [: 如果指针指向的单元值为零，向后跳转到对应的]指令的次一指令处
const OPCODE_RSB: u8     = 0x5d; // ]: 如果指针指向的单元值不为零，向前跳转到对应的[指令的次一指令处
const OPCODE_GETCHAR: u8 = 0x2c; // ,: 输入内容到指针指向的单元（ASCII码）
const OPCODE_PUTCHAR: u8 = 0x2e; // .: 输出指针指向的单元内容（ASCII码）

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
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
            OPCODE_SHL => Opcode::SHL,
            OPCODE_SHR => Opcode::SHR,
            OPCODE_ADD => Opcode::ADD,
            OPCODE_SUB => Opcode::SUB,
            OPCODE_LSB => Opcode::LSB,
            OPCODE_RSB => Opcode::RSB,
            OPCODE_GETCHAR => Opcode::GETCHAR,
            OPCODE_PUTCHAR => Opcode::PUTCHAR,
            _ => panic!("Unsupported opcode {}", u),
        }
    }
}

lazy_static! {
    static ref OPCODE_DICT: Vec<u8> = vec![
        OPCODE_SHL,
        OPCODE_SHR,
        OPCODE_ADD,
        OPCODE_SUB,
        OPCODE_LSB,
        OPCODE_RSB,
        OPCODE_GETCHAR,
        OPCODE_PUTCHAR,
    ];
}

pub struct Code {
    opcodes: Vec<Opcode>,
    jump_table: collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let opcodes: Vec<Opcode> = data.iter()
            .filter(|x| OPCODE_DICT.contains(x))
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

    pub fn opcodes(&self) -> &Vec<Opcode> {
        &self.opcodes
    }

    pub fn jump_table(&self) -> &collections::HashMap<usize, usize> {
        &self.jump_table
    }
}

