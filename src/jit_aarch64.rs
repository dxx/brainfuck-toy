use std::io::prelude::*;

use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use super::jit::ItOpcode as ItOpcode;
use super::jit::Code as Code;

const MEMERY_SIZE: usize = 65536;

unsafe extern "C" fn getchar(c: *mut u8) {
    std::io::stdin()
        .read_exact(std::slice::from_raw_parts_mut(c, 1))
        .unwrap();
}

unsafe extern "C" fn putchar(c: *const u8) {
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

        let mut ops = dynasmrt::aarch64::Assembler::new()?;

        dynasm!(ops
            ; ->getchar:
            ; .qword getchar as _
            ; ->putchar:
            ; .qword putchar as _
        );

        let entry_point = ops.offset();

        dynasm!(ops
            ; .arch aarch64
            ; str x30, [sp, #-16]!
            ; stp x0, x1, [sp, #-16]!
            ; stp x2, x3, [sp, #-16]!
        );
        
        for opcode in it_opcodes {
            match opcode {
                ItOpcode::SHL(v) => dynasm!(ops
                    ; sub x0, x0, v
                ),
                ItOpcode::SHR(v) => dynasm!(ops
                    ; add x0, x0, v
                ),
                ItOpcode::ADD(v) => dynasm!(ops
                    ; ldrb w9, [x0]
                    ; add w9, w9, v as u32
                    ; tbz w9, 8, >fine
                    ;fine:
                    ; strb w9, [x0]
                ),
                ItOpcode::SUB(v) => dynasm!(ops
                    ; ldrb w9, [x0]
                    ; sub w9, w9, v as u32
                    ; tbz w9, 8, >fine
                    ;fine:
                    ; strb w9, [x0]
                ),
                ItOpcode::LSB(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    stack.push((l, r));
                    dynasm!(ops
                        ; ldrb w9, [x0]
                        ; cbz w9, => r
                        ; => l
                    )
                }
                ItOpcode::RSB(_) => {
                    let (l, r) = stack.pop().unwrap();
                    dynasm!(ops
                        ; ldrb w9, [x0]
                        ; cbnz w9, => l
                        ; => r
                    )
                }
                ItOpcode::GETCHAR => dynasm!(ops
                    ; str x0, [sp, #24]
                    ; ldr x9, ->getchar
                    ; blr x9
                    ; mov x9, x1
                    ; ldp x1, x0, [sp, #16]
                    ; ldp x2, x3, [sp]
                ),
                ItOpcode::PUTCHAR => dynasm!(ops
                    ; str x0, [sp, #24]
                    ; ldr x9, ->putchar
                    ; blr x9
                    ; mov x9, x1
                    ; ldp x1, x0, [sp, #16]
                    ; ldp x2, x3, [sp]
                ),
            }
        }

        dynasm!(ops
            ; add sp, sp, #32
            ; ldr x30, [sp], #16
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
