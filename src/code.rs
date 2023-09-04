pub struct Code;

impl Code {
    pub fn dest(ins: &str) -> String {
        match ins {
            "" => "000".to_string(),
            "M" => "001".to_string(),
            "D" => "010".to_string(),
            "DM" => "011".to_string(),
            "MD" => "011".to_string(),
            "A" => "100".to_string(),
            "AM" => "101".to_string(),
            "MA" => "101".to_string(),
            "AD" => "110".to_string(),
            "DA" => "110".to_string(),
            "ADM" => "111".to_string(),
            "AMD" => "111".to_string(),
            "DAM" => "111".to_string(),
            "DMA" => "111".to_string(),
            "MAD" => "111".to_string(),
            "MDA" => "111".to_string(),
            _ => panic!("unknow dest"),
        }
    }
    pub fn comp(ins: &str) -> String {
        match ins {
            "0" => "0101010".to_string(),
            "1" => "0111111".to_string(),
            "-1" => "0111010".to_string(),
            "D" => "0001100".to_string(),
            "A" => "0110000".to_string(),
            "!D" => "0001101".to_string(),
            "!A" => "0110001".to_string(),
            "D+1" => "0011111".to_string(),
            "A+1" => "0110111".to_string(),
            "D-1" => "0001110".to_string(),
            "A-1" => "0110010".to_string(),
            "D+A" => "0000010".to_string(),
            "A+D" => "0000010".to_string(),
            "D-A" => "0010011".to_string(),
            "A-D" => "0000111".to_string(),
            "D&A" => "0000000".to_string(),
            "D|A" => "0010101".to_string(),
            "M" => "1110000".to_string(),
            "!M" => "1110001".to_string(),
            "-M" => "1110011".to_string(),
            "M+1" => "1110111".to_string(),
            "M-1" => "1110010".to_string(),
            "D+M" => "1000010".to_string(),
            "M+D" => "1000010".to_string(),
            "D-M" => "1010011".to_string(),
            "M-D" => "1000111".to_string(),
            "D&M" => "1000000".to_string(),
            "D|M" => "1010101".to_string(),
            _ => panic!("unknow comp"),
        }
    }

    pub fn jump(ins: &str) -> String {
        match ins {
            "" => "000".to_string(),
            "JGT" => "001".to_string(),
            "JEQ" => "010".to_string(),
            "JGE" => "011".to_string(),
            "JLT" => "100".to_string(),
            "JNE" => "101".to_string(),
            "JLE" => "110".to_string(),
            "JMP" => "111".to_string(),
            _ => panic!("unknow jump"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dest() {
        assert_eq!(Code::dest(""), "000");
        assert_eq!(Code::dest("M"), "001");
        assert_eq!(Code::dest("D"), "010");
        assert_eq!(Code::dest("DM"), "011");
        assert_eq!(Code::dest("MD"), "011");
        assert_eq!(Code::dest("A"), "100");
        assert_eq!(Code::dest("AM"), "101");
        assert_eq!(Code::dest("MA"), "101");
        assert_eq!(Code::dest("AD"), "110");
        assert_eq!(Code::dest("DA"), "110");
        assert_eq!(Code::dest("ADM"), "111");
        assert_eq!(Code::dest("AMD"), "111");
        assert_eq!(Code::dest("DAM"), "111");
        assert_eq!(Code::dest("DMA"), "111");
        assert_eq!(Code::dest("MAD"), "111");
        assert_eq!(Code::dest("MDA"), "111");
    }

    #[test]
    fn test_comp() {
        assert_eq!(Code::comp("0"), "0101010");
        assert_eq!(Code::comp("1"), "0111111");
        assert_eq!(Code::comp("-1"), "0111010");
        assert_eq!(Code::comp("D"), "0001100");
        assert_eq!(Code::comp("A"), "0110000");
        assert_eq!(Code::comp("!D"), "0001101");
        assert_eq!(Code::comp("!A"), "0110001");
        assert_eq!(Code::comp("D+1"), "0011111");
        assert_eq!(Code::comp("A+1"), "0110111");
        assert_eq!(Code::comp("D-1"), "0001110");
        assert_eq!(Code::comp("A-1"), "0110010");
        assert_eq!(Code::comp("D+A"), "0000010");
        assert_eq!(Code::comp("A+D"), "0000010");
        assert_eq!(Code::comp("D-A"), "0010011");
        assert_eq!(Code::comp("A-D"), "0000111");
        assert_eq!(Code::comp("D&A"), "0000000");
        assert_eq!(Code::comp("D|A"), "0010101");
        assert_eq!(Code::comp("M"), "1110000");
        assert_eq!(Code::comp("!M"), "1110001");
        assert_eq!(Code::comp("-M"), "1110011");
        assert_eq!(Code::comp("M+1"), "1110111");
        assert_eq!(Code::comp("M-1"), "1110010");
        assert_eq!(Code::comp("D+M"), "1000010");
        assert_eq!(Code::comp("M+D"), "1000010");
        assert_eq!(Code::comp("D-M"), "1010011");
        assert_eq!(Code::comp("M-D"), "1000111");
        assert_eq!(Code::comp("D&M"), "1000000");
        assert_eq!(Code::comp("D|M"), "1010101");
    }

    #[test]
    fn test_jump() {
        assert_eq!(Code::jump(""), "000");
        assert_eq!(Code::jump("JGT"), "001");
        assert_eq!(Code::jump("JEQ"), "010");
        assert_eq!(Code::jump("JGE"), "011");
        assert_eq!(Code::jump("JLT"), "100");
        assert_eq!(Code::jump("JNE"), "101");
        assert_eq!(Code::jump("JLE"), "110");
        assert_eq!(Code::jump("JMP"), "111");
    }
}
