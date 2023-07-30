pub fn and_gate(a: bool, b: bool) -> bool {
    a && b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_0_case() {
        assert!(!and_gate(true, false))
    }

    #[test]
    fn test_0_1_case() {
        assert!(!and_gate(false, true))
    }

    #[test]
    fn test_1_1_case() {
        assert!(and_gate(true, true))
    }

    #[test]
    fn test_0_0_case() {
        assert!(!and_gate(false, false))
    }
}
