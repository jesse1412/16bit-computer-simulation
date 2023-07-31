use crate::gates::or;

fn or_16_gate(a: &[bool; 16], b: &[bool; 16]) -> [bool; 16] {
    [
        or::or_gate(a[0], b[0]),
        or::or_gate(a[1], b[1]),
        or::or_gate(a[2], b[2]),
        or::or_gate(a[3], b[3]),
        or::or_gate(a[4], b[4]),
        or::or_gate(a[5], b[5]),
        or::or_gate(a[6], b[6]),
        or::or_gate(a[7], b[7]),
        or::or_gate(a[8], b[8]),
        or::or_gate(a[9], b[9]),
        or::or_gate(a[10], b[10]),
        or::or_gate(a[11], b[11]),
        or::or_gate(a[12], b[12]),
        or::or_gate(a[13], b[13]),
        or::or_gate(a[14], b[14]),
        or::or_gate(a[15], b[15]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_bits_off_a_on_b() {
        assert_eq!(or_16_gate(&[false; 16], &[true; 16]), [true; 16]);
    }

    #[test]
    fn test_all_bits_off_b_on_a() {
        assert_eq!(or_16_gate(&[true; 16], &[false; 16]), [true; 16]);
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
            assert_eq!(input_bus, or_16_gate(&input_bus, &input_bus));
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
            assert_eq!(bus, or_16_gate(&bus, &bus));
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
            assert_eq!([true; 16], or_16_gate(&input_bus_a, &input_bus_b));
        }
    }
}
