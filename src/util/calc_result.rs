use std::collections::HashSet;
use crate::lotto::Lotto;

fn get_matched_nums(source: &[u8], target: &[u8]) -> (u8, Vec<bool>) {
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

    (count, matched_arr)
}

pub fn calc_result(source: &Lotto, target: &Lotto) -> (u8, [bool; 7]) {
    let mut matched_arr = [false; 7];

    let (red_count, red_matched_arr) = get_matched_nums(&source.red_arr, &target.red_arr);
    let (blue_count, blue_matched_arr) = get_matched_nums(&source.blue_arr, &target.blue_arr);

    matched_arr[6] = blue_matched_arr[0];
    for (index, &matched) in red_matched_arr.iter().enumerate() {
        matched_arr[index] = matched;
    }

    let mut level_arr = [0, 0, 0, 0, 5, 4, 2];
    if blue_count > 0 {
        level_arr = [6, 6, 6, 5, 4, 3, 1];
    }

    (level_arr[red_count as usize], matched_arr)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_match_success() {
        assert_eq!(
            (1, vec![false, false, true]),
            get_matched_nums(
                &[1, 2, 3],
                &[4, 3, 5],
            ),
        );

        assert_eq!(
            (3, vec![true, true, true]),
            get_matched_nums(
                &[1, 2, 3],
                &[2, 3, 1],
            ),
        );

        assert_eq!(
            (0, vec![false]),
            get_matched_nums(
                &[1],
                &[2],
            ),
        );

        assert_eq!(
            (1, vec![true]),
            get_matched_nums(
                &[1],
                &[1],
            ),
        );
    }

    #[test]
    fn result_level_test() {
        let lotto = Lotto::new("01,02,03,04,05,06-07");
        let case_arr = [
            ("01,02,03,04,05,06-07", 1),
            ("01,02,03,04,05,33-07", 3),
            ("01,02,03,04,32,33-07", 4),
            ("01,02,03,31,32,33-07", 5),
            ("01,02,30,31,32,33-07", 6),
            ("01,29,30,31,32,33-07", 6),
            ("28,29,30,31,32,33-07", 6),
            ("01,02,03,04,05,06-16", 2),
            ("01,02,03,04,05,33-16", 4),
            ("01,02,03,04,32,33-16", 5),
            ("01,02,03,31,32,33-16", 0),
            ("01,02,30,31,32,33-16", 0),
            ("01,29,30,31,32,33-16", 0),
            ("28,29,30,31,32,33-16", 0),
        ];
        for (case, level) in case_arr {
            let case_lotto = Lotto::new(case);
            let (case_level, _) = calc_result(&case_lotto, &lotto);
            assert_eq!(level, case_level);
        }
    }
}
