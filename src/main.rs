use clap::Parser;
use serde::Serialize;
use serde_json::to_string_pretty;

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

    #[arg(long)]
    from: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let args_json = to_string_pretty(&args).unwrap();
    println!("args: {}", args_json);

    util::divide();

    let lottos = util::get_inputs(&args);
    // for lotto in &lottos {
    //     println!("{}", lotto.format());
    // }

    // util::divide();

    let mut total_reward = 0;
    let ssq_result_arr = util::get_result(&args).await.unwrap();
    let skip_empty = ssq_result_arr.len() > 1;
    for ssq_result in &ssq_result_arr {
        let lotto_result = lotto::LottoResult::new(ssq_result);
        let reward = lotto_result.print(&lottos, skip_empty);
        total_reward += reward;
    }
    if ssq_result_arr.len() > 0 {
        println!("总计: {total_reward}元");
    }
}
