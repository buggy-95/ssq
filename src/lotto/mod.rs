use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::util::{calc_result, parse_lotto, parse_num};

#[derive(Serialize, Deserialize, Debug)]
struct SsqPrizeGrade {
    r#type: u8,
    typemoney: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SsqResult {
    pub code: String,
    pub date: String,
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

        pool.insert(0, 0);
        for pair in &ssq_result.prizegrades {
            if pair.r#type == 7 { continue }
            let money = parse_num(&pair.typemoney);
            pool.insert(pair.r#type, money);
        }

        LottoResult { result, code, date, pool }
    }

    pub fn calc(self: &Self, target: &Lotto) -> (usize, u32, [bool; 7]) {
        let (level, matched) = calc_result(target, &self.result);
        let reward = self.pool.get(&level).unwrap() * target.scale;

        (level.into(), reward, matched)
    }

    pub fn print(self: &Self, input_arr: &Vec<Lotto>, skip_empty: bool) -> u32 {
        let mut reward_count = 0;

        let level_cn = ['无', '一', '二', '三', '四', '五', '六'];

        let mut reward_str_arr: Vec<String> = vec![];

        for (index, lotto) in input_arr.iter().enumerate() {
            let (level, reward, matched_arr) = self.calc(lotto);
            let mut print_str = String::new();
            print_str.push_str(&format!("第{}注: ", index + 1));
            for (i, matched) in matched_arr.iter().enumerate() {
                if !matched { print_str.push('⚫') }
                else if i < 6 { print_str.push('🔴') }
                else { print_str.push('🔵') }
            }
            print_str.push('\t');
            print_str.push(level_cn[level]);
            if level > 0 { print_str.push_str("等奖") }
            print_str.push('\t');
            print_str.push_str(&format!("{:>width$}", reward, width = 9));
            print_str.push('元');
            if level > 0 || !skip_empty { reward_str_arr.push(print_str) }
            reward_count += reward;
        }

        if reward_str_arr.len() > 0 || !skip_empty {
            println!("{} {}\n{}", self.code, self.date, self.result.format());
        }

        for str in reward_str_arr { println!("{str}") }

        reward_count
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
