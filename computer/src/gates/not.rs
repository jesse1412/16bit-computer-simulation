pub fn not_gate(a: bool) -> bool {
    !a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!not_gate(true))
    }

    #[test]
    fn test_0() {
        assert!(not_gate(false))
    }
}
