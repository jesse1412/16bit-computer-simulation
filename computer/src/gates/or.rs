use crate::gates::{and, not};

pub fn or_gate(a: bool, b: bool) -> bool {
    not::not_gate(and::and_gate(not::not_gate(a), not::not_gate(b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_0_case() {
        assert!(or_gate(true, false))
    }

    #[test]
    fn test_0_1_case() {
        assert!(or_gate(false, true))
    }

    #[test]
    fn test_1_1_case() {
        assert!(or_gate(true, true))
    }

    #[test]
    fn test_0_0_case() {
        assert!(!or_gate(false, false))
    }
}
