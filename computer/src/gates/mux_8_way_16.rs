use crate::gates::{mux_16, mux_4_way_16};

#[allow(clippy::too_many_arguments)]
pub fn mux_8_way_16_gate(
    a: &[bool; 16],
    b: &[bool; 16],
    c: &[bool; 16],
    d: &[bool; 16],
    e: &[bool; 16],
    f: &[bool; 16],
    g: &[bool; 16],
    h: &[bool; 16],
    switch: &[bool; 3],
) -> [bool; 16] {
    mux_16::mux_16_gate(
        &mux_4_way_16::mux_4_way_16_gate(a, b, c, d, &[switch[0], switch[1]]),
        &mux_4_way_16::mux_4_way_16_gate(e, f, g, h, &[switch[0], switch[1]]),
        switch[2],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const BUS_SIZE: usize = 16;

    #[test]
    fn test_all_1_each_switch() {
        let switches = [
            [false; 3],
            [false, false, true],
            [false, true, false],
            [false, true, true],
            [true, false, false],
            [true, false, true],
            [true, true, false],
            [true, true, true],
        ];
        for switch in switches {
            assert_eq!(
                mux_8_way_16_gate(
                    &[true; BUS_SIZE],
                    &[true; BUS_SIZE],
                    &[true; BUS_SIZE],
                    &[true; BUS_SIZE],
                    &[true; BUS_SIZE],
                    &[true; BUS_SIZE],
                    &[true; BUS_SIZE],
                    &[true; BUS_SIZE],
                    &switch
                ),
                [true; BUS_SIZE]
            );
        }
    }

    #[test]
    fn test_all_0_each_switch() {
        let switches = [
            [false; 3],
            [false, false, true],
            [false, true, false],
            [false, true, true],
            [true, false, false],
            [true, false, true],
            [true, true, false],
            [true, true, true],
        ];
        for switch in switches {
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &switch
                ),
                [false; BUS_SIZE]
            );
        }
    }

    #[test]
    fn test_each_bit_in_first_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [false; 3];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &bus,
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }

    #[test]
    fn test_each_bit_in_second_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [true, false, false];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &bus,
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }

    #[test]
    fn test_each_bit_in_third_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [false, true, false];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &bus,
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }

    #[test]
    fn test_each_bit_in_fourth_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [true, true, false];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &bus,
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }

    #[test]
    fn test_each_bit_in_fifth_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [false, false, true];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &bus,
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }

    #[test]
    fn test_each_bit_in_sixth_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [true, false, true];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &bus,
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }

    #[test]
    fn test_each_bit_in_seventh_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [false, true, true];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &bus,
                    &[false; BUS_SIZE],
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }

    #[test]
    fn test_each_bit_in_eighth_switch() {
        const SWITCH_TO_TEST: [bool; 3] = [true, true, true];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_8_way_16_gate(
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &[false; BUS_SIZE],
                    &bus,
                    &SWITCH_TO_TEST
                ),
                bus
            );
        }
    }
}
