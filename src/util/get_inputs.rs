use crate::Args;
use crate::lotto::Lotto;

pub fn get_inputs(args: &Args) -> Vec<Lotto> {
    let mut vec_lotto: Vec<Lotto> = vec![];
    println!("in test: {:?}", args);

    match (&args.lotto, &args.file) {
        (Some(str), _) => vec_lotto.push(Lotto::new(str)),
        (None, Some(path)) => println!("{:?}", path),
        _ => panic!("请输入一组投注"),
    }

    vec_lotto
}
