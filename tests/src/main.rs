use std::f32::consts::PI;

use half::f16;

fn main() {
    let alpha = f16::from(PI);
    println!("{:?} {:?}", alpha, alpha.round())
}
