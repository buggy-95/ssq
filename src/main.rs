use std::time::Instant;

use clap::Parser;
use serde::Serialize;
// use serde_json::to_string_pretty;

mod lotto;
mod util;

#[derive(Serialize, Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    lotto: Option<String>,

    #[arg(short, long)]
    file: Option<std::path::PathBuf>,

    #[arg(short, long)]
    code: Option<String>,

    #[arg(short, long)]
    recent: Option<u32>,

    #[arg(long)]
    all: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    from: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // let args_json = to_string_pretty(&args).unwrap();
    // println!("args: {}", args_json);
    // util::divide();

    let time_1 = Instant::now();
    let lottos = util::get_inputs(&args);
    if args.verbose { println!("[time] (main)\t\tget input:\t\t{:?}", time_1.elapsed()) }
    // for lotto in &lottos { println!("{}", lotto.format()) }
    // util::divide();

    let time_2 = Instant::now();
    let mut total_reward = 0;
    let ssq_result_arr = util::get_result(&args).await.unwrap();
    if args.verbose { println!("[time] (main)\t\tget final result:\t{:?}", time_2.elapsed()) }

    let time_3 = Instant::now();
    let mut print_str_arr: Vec<String> = vec![];
    let skip_empty = ssq_result_arr.len() > 1;
    for ssq_result in &ssq_result_arr {
        let lotto_result = lotto::LottoResult::new(ssq_result);
        let (str_arr, reward) = lotto_result.format(&lottos, skip_empty);
        print_str_arr.extend_from_slice(&str_arr);
        total_reward += reward;
    }
    if args.verbose { println!("[time] (main)\t\tcalc:\t\t\t{:?}", time_3.elapsed()) }

    util::divide();
    for str in print_str_arr { println!("{str}") }
    if ssq_result_arr.len() > 0 { println!("总计: {total_reward}元") }
}
