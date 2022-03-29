use lazy_static::lazy_static;

pub const OPCODE_SHL: u8     = 0x3c; // <: 指针减一
pub const OPCODE_SHR: u8     = 0x3e; // >: 指针加一
pub const OPCODE_ADD: u8     = 0x2b; // +: 指针指向的字节的值加一
pub const OPCODE_SUB: u8     = 0x2d; // -: 指针指向的字节的值减一
pub const OPCODE_LSB: u8     = 0x5b; // [: 如果指针指向的单元值为零，向后跳转到对应的 ] 指令的次一指令处
pub const OPCODE_RSB: u8     = 0x5d; // ]: 如果指针指向的单元值不为零，向前跳转到对应的 [ 指令的次一指令处
pub const OPCODE_GETCHAR: u8 = 0x2c; // ,: 输入内容到指针指向的单元（ASCII码）
pub const OPCODE_PUTCHAR: u8 = 0x2e; // .: 输出指针指向的单元内容（ASCII码）

lazy_static! {
    pub static ref OPCODE_DICT: Vec<u8> = vec![
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
