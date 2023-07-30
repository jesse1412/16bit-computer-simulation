use crate::gates::{and, not};

pub fn nand_gate(a: bool, b: bool) -> bool {
    not::not_gate(and::and_gate(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_0_case() {
        assert!(nand_gate(true, false))
    }

    #[test]
    fn test_0_1_case() {
        assert!(nand_gate(false, true))
    }

    #[test]
    fn test_1_1_case() {
        assert!(!nand_gate(true, true))
    }

    #[test]
    fn test_0_0_case() {
        assert!(nand_gate(false, false))
    }
}
