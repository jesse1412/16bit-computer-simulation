use crate::gates::{and, not};

fn dmux_gate(a: bool, switch: bool) -> [bool; 2] {
    [
        and::and_gate(a, not::not_gate(switch)),
        and::and_gate(a, switch),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_switch_off_case() {
        assert_eq!([true, false], dmux_gate(true, false));
    }

    #[test]
    fn test_1_switch_on_case() {
        assert_eq!([false, true], dmux_gate(true, true));
    }

    #[test]
    fn test_0_switch_off_case() {
        assert_eq!([false, false], dmux_gate(false, false));
    }

    #[test]
    fn test_0_switch_on_case() {
        assert_eq!([false, false], dmux_gate(false, false));
    }
}
