use clnooms::extended::f80;

fn main() {
    let alpha = f80::from(1.);
    println!("{} {:?}", alpha, alpha.atan())
}
