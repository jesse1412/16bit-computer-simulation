use crate::gates::{and, mux_16};

/// Note, switch works as follows;
/// switch[0] = 0, switch[1] = 0 -> a
/// switch[0] = 1, switch[1] = 0 -> b
/// switch[0] = 0, switch[1] = 1 -> c
/// switch[0] = 1, switch[1] = 1 -> d
#[allow(clippy::too_many_arguments)]
pub fn mux_4_way_16_gate(
    a: &[bool; 16],
    b: &[bool; 16],
    c: &[bool; 16],
    d: &[bool; 16],
    switch: &[bool; 2],
) -> [bool; 16] {
    /* Everything must come from our nand,
    so we won't use any loops solutions here
    as we haven't invented looping yet. */
    mux_16::mux_16_gate(
        &mux_16::mux_16_gate(&mux_16::mux_16_gate(a, b, switch[0]), c, switch[1]),
        d,
        and::and_gate(switch[0], switch[1]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const BUS_SIZE: usize = 16;

    #[test]
    fn test_all_1_each_switch() {
        let switches = [[false; 2], [false, true], [true, false], [true, true]];
        for switch in switches {
            assert_eq!(
                mux_4_way_16_gate(
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
        let switches = [[false; 2], [false, true], [true, false], [true, true]];
        for switch in switches {
            assert_eq!(
                mux_4_way_16_gate(
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
        const SWITCH_TO_TEST: [bool; 2] = [false; 2];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_4_way_16_gate(
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
    fn test_each_bit_in_second_switch() {
        const SWITCH_TO_TEST: [bool; 2] = [true, false];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_4_way_16_gate(
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
    fn test_each_bit_in_third_switch() {
        const SWITCH_TO_TEST: [bool; 2] = [false, true];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_4_way_16_gate(
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
    fn test_each_bit_in_fourth_switch() {
        const SWITCH_TO_TEST: [bool; 2] = [true; 2];
        for bit in 0..BUS_SIZE {
            let mut bus: [bool; BUS_SIZE] = [false; BUS_SIZE];
            bus[bit] = true;
            assert_eq!(
                mux_4_way_16_gate(
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
