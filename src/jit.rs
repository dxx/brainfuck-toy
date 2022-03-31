use dynasm::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use super::opcode;

use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum ItOpcode {
    SHL(u32), // SHL(10): 指针减 10
    SHR(u32), // SHR(10): 指针加 10
    ADD(u8),  // ADD(10): 指针指向的字节的值加 10
    SUB(u8),  // SUB(10): 指针指向的字节的值减 10
    LSB(u32), // LSB(10): 如果指针指向的单元值为零，跳转到 10 处
    RSB(u32), // RSB(0): 如果指针指向的单元值不为零，跳转到 0 处
    GETCHAR,
    PUTCHAR,
}

struct Code {
    it_opcodes: Vec<ItOpcode>, // 中间表优化，去掉重复指令的 opcodes
}

impl Code {
    fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let data: Vec<&u8> = data.iter()
            .filter(|x| opcode::OPCODE_DICT.contains(x))
            .collect();

        let mut it_opcodes = Vec::new();
        let mut stack = Vec::new();

        for u in data {
            match *u {
                opcode::OPCODE_SHL => match it_opcodes.last_mut() {
                    Some(ItOpcode::SHL(v)) => {
                        *v += 1;
                    }
                    _ => {
                        it_opcodes.push(ItOpcode::SHL(1));
                    }
                }
                opcode::OPCODE_SHR => match it_opcodes.last_mut() {
                    Some(ItOpcode::SHR(v)) => {
                        *v += 1;
                    }
                    _ => {
                        it_opcodes.push(ItOpcode::SHR(1));
                    }
                }
                opcode::OPCODE_ADD => match it_opcodes.last_mut() {
                    Some(ItOpcode::ADD(x)) => {
                        let v = x.overflowing_add(1).0;
                        *x = v;
                    }
                    _ => {
                        it_opcodes.push(ItOpcode::ADD(1));
                    }
                }
                opcode::OPCODE_SUB => match it_opcodes.last_mut() {
                    Some(ItOpcode::SUB(x)) => {
                        let v = x.overflowing_add(1).0;
                        *x = v;
                    }
                    _ => {
                        it_opcodes.push(ItOpcode::SUB(1));
                    }
                }
                opcode::OPCODE_LSB => {
                    it_opcodes.push(ItOpcode::LSB(0)); // 先存入 LSB(0)
                    stack.push(it_opcodes.len() - 1); // 记录 [ 所在的位置
                }
                opcode::OPCODE_RSB => {
                    let i = stack.pop().ok_or("Pop from empty stack")?;
                    it_opcodes.push(ItOpcode::RSB(i as u32));
                    let len = it_opcodes.len() - 1; // 最近的 ] 所在的位置
                    if let ItOpcode::LSB(v) = &mut it_opcodes[i] {
                        *v = (len - 1) as u32; // 修改 [ 配对的 ] 所在的位置
                    }
                }
                opcode::OPCODE_GETCHAR => {
                    it_opcodes.push(ItOpcode::GETCHAR);
                }
                opcode::OPCODE_PUTCHAR => {
                    it_opcodes.push(ItOpcode::PUTCHAR);
                }
                _ => panic!("Unsupported opcode {}", u),
            }
        }

        Ok(Code { it_opcodes })
    }
}

unsafe extern "sysv64" fn getchar(char: *mut u8) {
    std::io::stdin().read_exact(std::slice::from_raw_parts_mut(char, 1)).unwrap();
}

unsafe extern "sysv64" fn putchar(char: u8) {
    std::io::stdout().write_all(&[char]).unwrap();
}

#[derive(Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let it_opcodes = code.it_opcodes;
        let mut stack = Vec::new();

        let mut ops = dynasmrt::x64::Assembler::new()?;
        let entry_point = ops.offset();

        dynasm!(ops
            ; .arch x64
            ; mov rcx, rdi
        );
        
        for opcode in it_opcodes {
            match opcode {
                ItOpcode::SHL(v) => dynasm!(ops
                    ; sub rcx, v as i32
                ),
                ItOpcode::SHR(v) => dynasm!(ops
                    ; add rcx, v as i32
                ),
                ItOpcode::ADD(v) => dynasm!(ops
                    ; add BYTE [rcx], v as i8
                ),
                ItOpcode::SUB(v) => dynasm!(ops
                    ; sub BYTE [rcx], v as i8
                ),
                ItOpcode::LSB(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    stack.push((l, r));
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jz => r
                        ; => l
                    )
                }
                ItOpcode::RSB(_) => {
                    let (l, r) = stack.pop().unwrap();
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jnz => l
                        ; => r
                    )
                }
                ItOpcode::GETCHAR => dynasm!(ops
                    ; mov r12, rcx
                    ; mov rdi, rcx
                    ; mov rax, QWORD getchar as _
                    ; sub rsp, BYTE 0x28
                    ; call rax
                    ; add rsp, BYTE 0x28
                    ; mov rcx, r12
                ),
                ItOpcode::PUTCHAR => dynasm!(ops
                    ; mov r12, rcx
                    ; mov rdi, [rcx]
                    ; mov rax, QWORD putchar as _
                    ; sub rsp, BYTE 0x28
                    ; call rax
                    ; add rsp, BYTE 0x28
                    ; mov rcx, r12
                ),
            }
        }

        dynasm!(ops
            ; ret
        );

        let exec_buffer = ops.finalize().unwrap();
        let mut memory: Box<[u8]> = vec![0; 65536].into_boxed_slice();
        let memory_addr_from = memory.as_mut_ptr();
        let memory_addr_to = unsafe { memory_addr_from.add(memory.len()) };
        let fun: fn(memory_addr_from: *mut u8, memory_addr_to: *mut u8) =
            unsafe { std::mem::transmute(exec_buffer.ptr(entry_point)) };
        fun(memory_addr_from, memory_addr_to);

        Ok(())
    }
}
