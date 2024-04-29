use ssq::*;

fn main() {
    let str = "01,02,03,04,05";
    let (red, blue, scale) = parse_lotto(str);
    println!("red: {:?}", red);
    println!("blue: {:?}", blue);
    println!("scale: {:?}", scale);
}
