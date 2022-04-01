use super::opcode;

#[derive(Debug, PartialEq, Eq)]
pub enum ItOpcode {
    SHL(u32), // SHL(10): 指针减 10
    SHR(u32), // SHR(10): 指针加 10
    ADD(u8),  // ADD(10): 指针指向的字节的值加 10
    SUB(u8),  // SUB(10): 指针指向的字节的值减 10
    LSB(u32), // LSB(10): 如果指针指向的单元值为零，跳转到 10 处
    RSB(u32), // RSB(0): 如果指针指向的单元值不为零，跳转到 0 处
    GETCHAR,
    PUTCHAR,
}

pub struct Code {
    pub it_opcodes: Vec<ItOpcode>, // 中间表优化，去掉重复指令的 opcodes
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
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
