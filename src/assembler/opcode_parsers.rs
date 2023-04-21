use nom::types::CompleteStr;
use nom::*;
use crate::assembler::Token;
use crate::instruction::Opcode;

named!(pub opcode<CompleteStr, Token>,
    do_parse!(
        //tag!("load") >> (Token::Op{code: Opcode::LOAD})
        opcode: alpha1 >>
        (
            Token::Op {code: Opcode::from(opcode)}
        )
    )
);


mod tests {
    #![allow(unused_imports)]
    use super::opcode;
    use assembler::Token;
    use instruction::Opcode;
    use nom::types::CompleteStr;
    use crate::{assembler, instruction};


    #[test]
    fn test_opcode() {
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOAD});
        assert_eq!(rest, CompleteStr(""));

        let result = opcode(CompleteStr("aold"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });

    }
}