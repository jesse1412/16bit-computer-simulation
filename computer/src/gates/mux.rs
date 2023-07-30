use crate::gates::{and, not, or};

pub fn mux_gate(a: bool, b: bool, switch: bool) -> bool {
    or::or_gate(
        and::and_gate(a, not::not_gate(switch)),
        and::and_gate(b, switch),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_0_switch_off_case() {
        assert!(mux_gate(true, false, false))
    }

    #[test]
    fn test_0_1_switch_off_case() {
        assert!(!mux_gate(false, true, false))
    }

    #[test]
    fn test_1_1_switch_off_case() {
        assert!(mux_gate(true, true, false))
    }

    #[test]
    fn test_0_0_switch_off_case() {
        assert!(!mux_gate(false, false, false))
    }

    #[test]
    fn test_1_0_switch_on_case() {
        assert!(!mux_gate(true, false, true))
    }

    #[test]
    fn test_0_1_switch_on_case() {
        assert!(mux_gate(false, true, true))
    }

    #[test]
    fn test_1_1_switch_on_case() {
        assert!(mux_gate(true, true, true))
    }

    #[test]
    fn test_0_0_switch_on_case() {
        assert!(!mux_gate(false, false, true))
    }
}
