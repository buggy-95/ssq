use std::collections::HashSet;
use crate::lotto::Lotto;

fn get_matched_nums(source: &[u8], target: &[u8]) -> (Vec<bool>, u8) {
    let mut count: u8 = 0;
    let mut matched_arr: Vec<bool> = vec![];
    let mut num_sets: HashSet<u8> = HashSet::new();

    for num in target {
        num_sets.insert(*num);
    }

    for num in source {
        if num_sets.contains(num) {
            matched_arr.push(true);
            count += 1;
        } else {
            matched_arr.push(false);
        }
    }

    (matched_arr, count)
}

pub fn calc_result(source: &Lotto, target: &Lotto) {
    let mut red_match_result = get_matched_nums(&source.red_arr, &target.red_arr);
    let blue_match_result = get_matched_nums(&source.blue_arr, &target.blue_arr);

    let red_match_count = red_match_result.1;
    let blue_match_count = blue_match_result.1;
    red_match_result.0.extend(blue_match_result.0);

    println!("red: {red_match_count}");
    println!("blue: {blue_match_count}");
    println!("matched: {:?}", red_match_result.0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_match_success() {
        assert_eq!(
            (vec![false, false, true], 1),
            get_matched_nums(
                &[1, 2, 3],
                &[4, 3, 5],
            ),
        );

        assert_eq!(
            (vec![true, true, true], 3),
            get_matched_nums(
                &[1, 2, 3],
                &[2, 3, 1],
            ),
        );

        assert_eq!(
            (vec![false], 0),
            get_matched_nums(
                &[1],
                &[2],
            ),
        );

        assert_eq!(
            (vec![true], 1),
            get_matched_nums(
                &[1],
                &[1],
            ),
        );
    }
}
