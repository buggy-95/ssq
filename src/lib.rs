#[allow(dead_code)]
#[derive(Debug)]
pub struct Lotto {
    red_arr: [u8; 6],
    blue_arr: [u8; 1],
    scale: u32,
}

impl Lotto {
    pub fn new(str: &str) -> Lotto {
        let (red_arr, blue_arr, scale) = parse_lotto(str);
        Lotto { red_arr, blue_arr, scale }
    }
}

pub fn parse_lotto(str: &str) -> ([u8; 6], [u8; 1], u32) {
    let mut red_arr: [u8; 6] = [0; 6];
    let mut blue_arr: [u8; 1] = [0];
    let mut scale: u32 = 1;

    let mut red_record_arr: [bool; 34] = [false; 34];
    let mut num_index: usize = 0;
    let mut num_str = String::new();
    let mut append_num = |num_str: &mut String, char: char| {
        match num_str.parse::<u8>() {
            Err(_) => panic!("解析错误: {str}"),
            Ok(num) => match num_index {
                0..=5 => {
                    if num_index < 5 && char != ',' { panic!("红球数量错误") }
                    if num_index == 5 && char != '-' { panic!("红球数量错误") }
                    if num < 1 { panic!("红球最小为1") }
                    if num > 33 { panic!("红球最大为33") }
                    if red_record_arr[num as usize] { panic!("红球重复: {num}") }
                    red_arr[num_index] = num;
                    red_record_arr[num as usize] = true;
                },
                6 => {
                    if !"x *\0".contains(char) { panic!("解析错误: {str}") }
                    if num < 1 { panic!("蓝球最小为1") }
                    if num > 16 { panic!("蓝球最大为16") }
                    blue_arr[0] = num;
                },
                7 => scale = num as u32,
                _ => {}
            }
        }
        num_str.clear();
        num_index += 1;
    };

    for char in str.chars() {
        match char {
            '0'..='9' => num_str.push(char),
            ',' | '-' | 'x' | ' ' => append_num(&mut num_str, char),
            _ => println!("不应出现的字符: {char}"),
        }
    }

    append_num(&mut num_str, '\0');

    print!("\n");

    (red_arr, blue_arr, scale)
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_lotto_success() {
        assert_eq!(
            parse_lotto("01,02,03,04,05,06-07 2"),
            ([1, 2, 3, 4, 5, 6], [7], 2),
        );
        assert_eq!(
            parse_lotto("01,02,03,04,05,06-07"),
            ([1, 2, 3, 4, 5, 6], [7], 1),
        );
    }

    #[test]
    #[should_panic(expected = "红球最大为33")]
    fn parse_error_1() {
        parse_lotto("34,02,03,04,05,06-07 2");
    }

    #[test]
    #[should_panic(expected = "红球最小为1")]
    fn parse_error_2() {
        parse_lotto("00,02,03,04,05,06-07 2");
    }

    #[test]
    #[should_panic(expected = "红球重复:")]
    fn parse_error_3() {
        parse_lotto("02,02,03,04,05,06-07 2");
    }

    #[test]
    #[should_panic(expected = "蓝球最大为16")]
    fn parse_error_4() {
        parse_lotto("01,02,03,04,05,06-17 2");
    }

    #[test]
    #[should_panic(expected = "蓝球最小为1")]
    fn parse_error_5() {
        parse_lotto("01,02,03,04,05,06-00 2");
    }

    #[test]
    #[should_panic(expected = "解析错误:")]
    fn parse_error_6() {
        parse_lotto("-01,02,03,04,05,06-17 2");
    }

    #[test]
    #[should_panic(expected = "解析错误:")]
    fn parse_error_7() {
        parse_lotto("01,-2,03,04,05,06-17 2");
    }

    #[test]
    #[should_panic(expected = "解析错误:")]
    fn parse_error_8() {
        parse_lotto("01,02,03,04,05,06-07-");
    }

    #[test]
    #[should_panic(expected = "解析错误:")]
    fn parse_error_9() {
        parse_lotto("01,02,03,04,05,06-01,02");
    }

    #[test]
    #[should_panic(expected = "红球数量错误")]
    fn parse_error_10() {
        parse_lotto("01,02,03,04,05,06,07-08 2");
    }

    #[test]
    #[should_panic(expected = "红球数量错误")]
    fn parse_error_11() {
        parse_lotto("01,02,03,04,05-06 2");
    }
}
