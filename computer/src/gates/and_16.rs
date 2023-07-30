use crate::gates::and;

pub fn and_16_gate(bus_a: &[bool; 16], bus_b: &[bool; 16]) -> [bool; 16] {
    [
        and::and_gate(bus_a[0], bus_b[0]),
        and::and_gate(bus_a[1], bus_b[1]),
        and::and_gate(bus_a[2], bus_b[2]),
        and::and_gate(bus_a[3], bus_b[3]),
        and::and_gate(bus_a[4], bus_b[4]),
        and::and_gate(bus_a[5], bus_b[5]),
        and::and_gate(bus_a[6], bus_b[6]),
        and::and_gate(bus_a[7], bus_b[7]),
        and::and_gate(bus_a[8], bus_b[8]),
        and::and_gate(bus_a[9], bus_b[9]),
        and::and_gate(bus_a[10], bus_b[10]),
        and::and_gate(bus_a[11], bus_b[11]),
        and::and_gate(bus_a[12], bus_b[12]),
        and::and_gate(bus_a[13], bus_b[13]),
        and::and_gate(bus_a[14], bus_b[14]),
        and::and_gate(bus_a[15], bus_b[15]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_bits_off_a_on_b() {
        assert_eq!(and_16_gate(&[false; 16], &[true; 16]), [false; 16]);
    }

    #[test]
    fn test_all_bits_off_b_on_a() {
        assert_eq!(and_16_gate(&[true; 16], &[false; 16]), [false; 16]);
    }

    #[test]
    fn test_each_bit_pair_same_on_alone() {
        /* Each bit pair in bus a and b are equal.
        One bit in each bus is on, the rest are off.
        This is ran over all 16 bit pairs. */
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus = [false; INPUT_BUS_SIZE];
            input_bus[i] = true;
            assert_eq!(input_bus, and_16_gate(&input_bus, &input_bus));
        }
    }

    #[test]
    fn test_each_bit_pair_same_off_alone() {
        /* Each bit pair in bus a and b are equal.
        One bit in each bus is off, the rest are on.
        This is ran over all 16 bit pairs. */
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut bus = [true; INPUT_BUS_SIZE];
            bus[i] = false;
            assert_eq!(bus, and_16_gate(&bus, &bus));
        }
    }

    #[test]
    fn test_each_bit_pair_different() {
        /* Each bit pair in bus a and b are opposite.
        One bit in bus a is on, all others are off.
        One bit in bus b is off, all others are on.
        This is ran over all 16 bit pairs. */
        const INPUT_BUS_SIZE: usize = 16;
        for i in 0..INPUT_BUS_SIZE {
            let mut input_bus_a = [false; INPUT_BUS_SIZE];
            input_bus_a[i] = true;
            let input_bus_b = input_bus_a.map(|x| !x);
            assert_eq!([false; 16], and_16_gate(&input_bus_a, &input_bus_b));
        }
    }
}
