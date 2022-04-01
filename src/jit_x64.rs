use std::io::prelude::*;

use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use super::jit::ItOpcode as ItOpcode;
use super::jit::Code as Code;

const MEMERY_SIZE: usize = 65536;

unsafe extern "sysv64" fn getchar(c: *mut u8) {
    std::io::stdin()
        .read_exact(std::slice::from_raw_parts_mut(c, 1))
        .unwrap();
}

unsafe extern "sysv64" fn putchar(c: *const u8) {
    std::io::stdout()
        .write_all(std::slice::from_raw_parts(c, 1))
        .unwrap();
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
            ; sub rsp, BYTE 0x28
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
                },
                ItOpcode::RSB(_) => {
                    let (l, r) = stack.pop().unwrap();
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jnz => l
                        ; => r
                    )
                },
                ItOpcode::GETCHAR => dynasm!(ops
                    ; mov r12, rcx
                    ; mov rdi, rcx
                    ; mov rax, QWORD getchar as _
                    ; call rax
                    ; mov rcx, r12
                ),
                ItOpcode::PUTCHAR => dynasm!(ops
                    ; mov r12, rcx
                    ; mov rdi, rcx
                    ; mov rax, QWORD putchar as _
                    ; call rax
                    ; mov rcx, r12
                ),
            }
        }

        dynasm!(ops
            ; add rsp, BYTE 0x28
            ; ret
        );

        let exec_buffer = ops.finalize().unwrap();
        let mut memory: Box<[u8]> = vec![0; MEMERY_SIZE].into_boxed_slice();
        let memory_addr_from = memory.as_mut_ptr();
        let memory_addr_to = unsafe { memory_addr_from.add(memory.len()) };
        let fun: fn(memory_addr_from: *mut u8, memory_addr_to: *mut u8) =
            unsafe { std::mem::transmute(exec_buffer.ptr(entry_point)) };
        fun(memory_addr_from, memory_addr_to);

        Ok(())
    }
}
