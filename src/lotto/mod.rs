use crate::util::parse_lotto;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lotto {
    red_arr: [u8; 6],
    blue_arr: [u8; 1],
    scale: u32,
}

#[allow(dead_code)]
impl Lotto {
    pub fn new(str: &str) -> Lotto {
        let (red_arr, blue_arr, scale) = parse_lotto(str);
        Lotto { red_arr, blue_arr, scale }
    }
}
