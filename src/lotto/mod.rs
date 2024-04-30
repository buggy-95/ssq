use serde::Serialize;
use crate::util::parse_lotto;

#[derive(Serialize, Debug)]
pub struct Lotto {
    pub red_arr: [u8; 6],
    pub blue_arr: [u8; 1],
    pub scale: u32,
}

impl Lotto {
    pub fn new(str: &str) -> Lotto {
        let (red_arr, blue_arr, scale) = parse_lotto(str);
        Lotto { red_arr, blue_arr, scale }
    }

    pub fn format(self: &Self) -> String {
        let mut formated = String::new();

        for red in self.red_arr {
            if red < 10 {
                formated.push('0');
            }
            formated.push_str(&red.to_string());
        }

        formated.push('-');

        let blue = self.blue_arr[0];
        if blue < 10 {
            formated.push('0');
        }
        formated.push_str(&blue.to_string());

        if self.scale > 1 {
            formated.push('x');
            formated.push_str(&self.scale.to_string());
        }

        formated
    }
}

#[allow(dead_code)]
pub struct LottoResult {
    result: Lotto,
    code: String,
    date: String,
}

impl LottoResult {
    pub fn calc(self: &Self, target: &Lotto) {}
}
