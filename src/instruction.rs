#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode{
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    IGL,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GTE,
    LTE,
    LT,
    GT,
    JMPE,
    NOP,
}


#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction{
        Instruction{
            opcode: opcode
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GTE,
            12 => Opcode::GT,
            13 => Opcode::LTE,
            14 => Opcode::LT,
            15 => Opcode::JMPE,
            16 => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn  test_create_instruction(){
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT)
    }
}