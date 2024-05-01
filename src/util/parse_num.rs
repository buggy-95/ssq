pub fn parse_num(str: &str) -> u32 {
    let mut valid_str = String::new();
    for char in str.chars() {
        if char.is_digit(10) { valid_str.push(char) }
        else { break }
    }

    valid_str.parse::<u32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_typemoney() {
        assert_eq!(
            5250000,
            parse_num("5250000（含加奖250000）"),
        );
    }
}

