use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::util::{parse_lotto, calc_result};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct SsqPrizeGrade {
    r#type: u8,
    typemoney: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SsqResult {
    code: String,
    date: String,
    red: String,
    blue: String,
    prizegrades: [SsqPrizeGrade; 7],
}

#[derive(Serialize, Debug)]
pub struct Lotto {
    pub red_arr: [u8; 6],
    pub blue_arr: [u8; 1],
    pub scale: u32,
}

impl Lotto {
    pub fn new(str: &str) -> Self {
        let (red_arr, blue_arr, scale) = parse_lotto(str);
        Lotto { red_arr, blue_arr, scale }
    }

    #[allow(dead_code)]
    pub fn format(self: &Self) -> String {
        let num_to_str = |num: u8| format!("{:02}", num);
        let mut formated = self.red_arr.map(num_to_str).join(",");

        formated.push('-');

        let blue = self.blue_arr[0];
        formated.push_str(&num_to_str(blue));

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
    pool: HashMap<u8, u32>,
}

impl LottoResult {
    pub fn new(ssq_result: &SsqResult) -> Self {
        let mut pool: HashMap<u8, u32> = HashMap::new();
        let code = ssq_result.code.clone();
        let date = ssq_result.date.clone();
        let ssq_str = format!("{}-{}", ssq_result.red, ssq_result.blue);
        let result = Lotto::new(&ssq_str);

        for pair in &ssq_result.prizegrades {
            if pair.r#type == 7 { continue }
            let money = pair.typemoney.parse::<u32>().unwrap();
            pool.insert(pair.r#type, money);
        }

        LottoResult { result, code, date, pool }
    }

    pub fn calc(self: &Self, target: &Lotto) -> (usize, u32, [bool; 7]) {
        let (level, matched) = calc_result(target, &self.result);
        let reward = self.pool.get(&level).unwrap() * target.scale;
        println!("当前中奖: {level}, {reward}");
        println!("{:?}", matched);

        (level.into(), reward, matched)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_test() {
        assert_eq!(
            "01,02,03,04,05,06-07",
            Lotto::new("1,2,3,4,5,6-7").format(),
        );
        assert_eq!(
            "01,02,03,04,05,06-07",
            Lotto::new("1,2,3,4,5,6-7x1").format(),
        );
        assert_eq!(
            "01,02,03,04,05,06-07x2",
            Lotto::new("1,2,3,4,5,6-7x2").format(),
        );
    }
}
