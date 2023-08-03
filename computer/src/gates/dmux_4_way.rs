use crate::gates::dmux::dmux_gate;

pub fn dmux_4_way_gate(a: bool, switch: &[bool; 2]) -> (bool, bool, bool, bool) {
    let (gate_1_out_0, gate_1_out_1) = dmux_gate(a, switch[1]);
    let (gate_2_out_0, gate_2_out_1) = dmux_gate(gate_1_out_0, switch[0]);
    let (gate_3_out_0, gate_3_out_1) = dmux_gate(gate_1_out_1, switch[0]);
    (gate_2_out_0, gate_2_out_1, gate_3_out_0, gate_3_out_1)
    // dmux_gate(dmux_gate(a, switch[0]), switch[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_1_each_switch() {
        const EXPECTED_OUTPUTS: [(bool, bool, bool, bool); 4] = [
            (true, false, false, false),
            (false, true, false, false),
            (false, false, true, false),
            (false, false, false, true),
        ];
        const SWITCHES: [[bool; 2]; 4] = [[false; 2], [true, false], [false, true], [true, true]];

        for (switch, expected) in SWITCHES.iter().zip(EXPECTED_OUTPUTS.iter()) {
            assert_eq!(dmux_4_way_gate(true, switch), *expected);
        }
    }

    #[test]
    fn test_all_0_each_switch() {
        const EXPECTED_OUTPUTS: [(bool, bool, bool, bool); 4] = [
            (false, false, false, false),
            (false, false, false, false),
            (false, false, false, false),
            (false, false, false, false),
        ];
        const SWITCHES: [[bool; 2]; 4] = [[false; 2], [true, false], [false, true], [true, true]];

        for (switch, expected) in SWITCHES.iter().zip(EXPECTED_OUTPUTS.iter()) {
            assert_eq!(dmux_4_way_gate(false, switch), *expected);
        }
    }
}
