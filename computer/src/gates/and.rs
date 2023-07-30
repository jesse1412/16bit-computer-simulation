pub fn gate_and(a: bool, b: bool) -> bool {
    a && b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_0_case() {
        assert!(!gate_and(true, false))
    }

    #[test]
    fn test_0_1_case() {
        assert!(!gate_and(false, true))
    }

    #[test]
    fn test_1_1_case() {
        assert!(gate_and(true, true))
    }

    #[test]
    fn test_0_0_case() {
        assert!(!gate_and(false, false))
    }
}
