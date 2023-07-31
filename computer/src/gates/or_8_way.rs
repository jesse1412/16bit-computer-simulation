use crate::gates::or;

#[allow(clippy::too_many_arguments)]
pub fn or_8_way_gate(
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
) -> bool {
    or::or_gate(
        a,
        or::or_gate(
            b,
            or::or_gate(
                c,
                or::or_gate(d, or::or_gate(e, or::or_gate(f, or::or_gate(g, h)))),
            ),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_on() {
        assert!(or_8_way_gate(
            true, true, true, true, true, true, true, true
        ))
    }

    #[test]
    fn test_all_off() {
        assert!(!or_8_way_gate(
            false, false, false, false, false, false, false, false
        ))
    }

    #[test]
    fn test_one_on_rest_off() {
        /* Tests or for each individual value being on alone. */
        const INPUT_BUS_SIZE: usize = 8;
        for i in 0..INPUT_BUS_SIZE {
            let mut inputs = [false; INPUT_BUS_SIZE];
            inputs[i] = true;
            assert!(or_8_way_gate(
                inputs[0], inputs[1], inputs[2], inputs[3], inputs[4], inputs[5], inputs[6],
                inputs[7]
            ));
        }
    }
}
