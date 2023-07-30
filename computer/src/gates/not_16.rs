use crate::gates::not;

pub fn not_16_gate(bus: &[bool; 16]) -> [bool; 16] {
    /* Everything must come from our nand,
    so we won't use any loops solutions here
    as we haven't invented looping yet. */
    [
        not::not_gate(bus[0]),
        not::not_gate(bus[1]),
        not::not_gate(bus[2]),
        not::not_gate(bus[3]),
        not::not_gate(bus[4]),
        not::not_gate(bus[5]),
        not::not_gate(bus[6]),
        not::not_gate(bus[7]),
        not::not_gate(bus[8]),
        not::not_gate(bus[9]),
        not::not_gate(bus[10]),
        not::not_gate(bus[11]),
        not::not_gate(bus[12]),
        not::not_gate(bus[13]),
        not::not_gate(bus[14]),
        not::not_gate(bus[15]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_bits_off() {
        assert_eq!(not_16_gate(&[false; 16]), [true; 16]);
    }

    #[test]
    fn test_all_bits_on() {
        assert_eq!(not_16_gate(&[true; 16]), [false; 16]);
    }

    #[test]
    fn test_each_bit_on_alone() {
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus = [false; INPUT_BUS_SIZE];
            input_bus[i] = true;
            let output_bus = input_bus.map(|x| !x);
            assert_eq!(output_bus, not_16_gate(&input_bus));
        }
    }

    #[test]
    fn test_each_bit_off_alone() {
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus = [true; INPUT_BUS_SIZE];
            input_bus[i] = false;
            let output_bus = input_bus.map(|x| !x);
            assert_eq!(output_bus, not_16_gate(&input_bus));
        }
    }
}
