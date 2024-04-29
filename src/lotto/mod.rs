use serde::Serialize;
use crate::util::parse_lotto;

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub struct Lotto {
    pub red_arr: [u8; 6],
    pub blue_arr: [u8; 1],
    pub scale: u32,
}

#[allow(dead_code)]
impl Lotto {
    pub fn new(str: &str) -> Lotto {
        let (red_arr, blue_arr, scale) = parse_lotto(str);
        Lotto { red_arr, blue_arr, scale }
    }
}
