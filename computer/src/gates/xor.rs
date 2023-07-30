use crate::gates::{and, nand, or};

pub fn xor_gate(a: bool, b: bool) -> bool {
    and::and_gate(or::or_gate(a, b), nand::nand_gate(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_0_case() {
        assert!(xor_gate(true, false))
    }

    #[test]
    fn test_0_1_case() {
        assert!(xor_gate(false, true))
    }

    #[test]
    fn test_1_1_case() {
        assert!(!xor_gate(true, true))
    }

    #[test]
    fn test_0_0_case() {
        assert!(!xor_gate(false, false))
    }
}
