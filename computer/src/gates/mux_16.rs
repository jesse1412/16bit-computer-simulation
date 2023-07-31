use crate::gates::mux;

pub fn mux_16_gate(a: &[bool; 16], b: &[bool; 16], switch: bool) -> [bool; 16] {
    [
        mux::mux_gate(a[0], b[0], switch),
        mux::mux_gate(a[1], b[1], switch),
        mux::mux_gate(a[2], b[2], switch),
        mux::mux_gate(a[3], b[3], switch),
        mux::mux_gate(a[4], b[4], switch),
        mux::mux_gate(a[5], b[5], switch),
        mux::mux_gate(a[6], b[6], switch),
        mux::mux_gate(a[7], b[7], switch),
        mux::mux_gate(a[8], b[8], switch),
        mux::mux_gate(a[9], b[9], switch),
        mux::mux_gate(a[10], b[10], switch),
        mux::mux_gate(a[11], b[11], switch),
        mux::mux_gate(a[12], b[12], switch),
        mux::mux_gate(a[13], b[13], switch),
        mux::mux_gate(a[14], b[14], switch),
        mux::mux_gate(a[15], b[15], switch),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_bits_off_a_on_b_switch_off() {
        assert_eq!(mux_16_gate(&[false; 16], &[true; 16], false), [false; 16]);
    }

    #[test]
    fn test_all_bits_off_b_on_a_switch_off() {
        assert_eq!(mux_16_gate(&[true; 16], &[false; 16], false), [true; 16]);
    }

    #[test]
    fn test_all_bits_off_a_on_b_switch_on() {
        assert_eq!(mux_16_gate(&[false; 16], &[true; 16], true), [true; 16]);
    }

    #[test]
    fn test_all_bits_off_b_on_a_switch_on() {
        assert_eq!(mux_16_gate(&[true; 16], &[false; 16], true), [false; 16]);
    }

    #[test]
    fn test_one_bit_on_a_all_off_b_switch_off() {
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus = [false; INPUT_BUS_SIZE];
            input_bus[i] = true;
            assert_eq!(mux_16_gate(&input_bus, &[false; 16], false), input_bus);
        }
    }

    #[test]
    fn test_one_bit_on_b_all_off_a_switch_off() {
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus = [false; INPUT_BUS_SIZE];
            input_bus[i] = true;
            assert_eq!(mux_16_gate(&[false; 16], &input_bus, false), [false; 16]);
        }
    }

    #[test]
    fn test_one_bit_on_a_all_off_b_switch_on() {
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus = [false; INPUT_BUS_SIZE];
            input_bus[i] = true;
            assert_eq!(mux_16_gate(&input_bus, &[false; 16], true), [false; 16]);
        }
    }

    #[test]
    fn test_one_bit_on_b_all_off_a_switch_on() {
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus = [false; INPUT_BUS_SIZE];
            input_bus[i] = true;
            assert_eq!(mux_16_gate(&[false; 16], &input_bus, true), input_bus);
        }
    }
}
