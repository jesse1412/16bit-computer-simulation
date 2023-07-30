use crate::gates::{and, not};

fn gate_nand(a: bool, b: bool) -> bool {
    not::gate_not(and::gate_and(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_0_case() {
        assert!(gate_nand(true, false))
    }

    #[test]
    fn test_0_1_case() {
        assert!(gate_nand(false, true))
    }

    #[test]
    fn test_1_1_case() {
        assert!(!gate_nand(true, true))
    }

    #[test]
    fn test_0_0_case() {
        assert!(gate_nand(false, false))
    }
}
