use crate::Args;
use crate::lotto::Lotto;
use super::*;

pub fn get_inputs(args: &Args) -> Vec<Lotto> {
    let mut vec_lotto: Vec<Lotto> = vec![];

    match (&args.lotto, &args.file) {
        (Some(str), _) => vec_lotto.push(Lotto::new(str)),
        (None, Some(path)) => {
            for line in read_file(path).unwrap() {
                vec_lotto.push(Lotto::new(&line));
            }
        },
        _ => panic!("请输入一组投注"),
    }

    vec_lotto
}
