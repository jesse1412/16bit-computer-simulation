mod gates;

use gates::mux_8_way_16;

fn main() {
    println!("Hello, world!");
    let a = mux_8_way_16::mux_8_way_16_gate(
        &[true; 16],
        &[true; 16],
        &[true; 16],
        &[true; 16],
        &[true; 16],
        &[true; 16],
        &[true; 16],
        &[true; 16],
        &[true; 3],
    );
}
