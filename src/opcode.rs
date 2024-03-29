use once_cell::sync::Lazy;

pub const OPCODE_SHL: u8     = b'<'; // <: 指针减一
pub const OPCODE_SHR: u8     = b'>'; // >: 指针加一
pub const OPCODE_ADD: u8     = b'+'; // +: 指针指向的字节的值加一
pub const OPCODE_SUB: u8     = b'-'; // -: 指针指向的字节的值减一
pub const OPCODE_LSB: u8     = b'['; // [: 如果指针指向的单元值为零，向后跳转到对应的 ] 指令的次一指令处
pub const OPCODE_RSB: u8     = b']'; // ]: 如果指针指向的单元值不为零，向前跳转到对应的 [ 指令的次一指令处
pub const OPCODE_GETCHAR: u8 = b','; // ,: 输入内容到指针指向的单元（ASCII码）
pub const OPCODE_PUTCHAR: u8 = b'.'; // .: 输出指针指向的单元内容（ASCII码）

pub static OPCODE_DICT: Lazy<Vec<u8>> = Lazy::new(|| vec![
    OPCODE_SHL,
    OPCODE_SHR,
    OPCODE_ADD,
    OPCODE_SUB,
    OPCODE_LSB,
    OPCODE_RSB,
    OPCODE_GETCHAR,
    OPCODE_PUTCHAR,
]);
