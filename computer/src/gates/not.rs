pub fn gate_not(a: bool) -> bool {
    !a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!gate_not(true))
    }

    #[test]
    fn test_0() {
        assert!(gate_not(false))
    }
}
