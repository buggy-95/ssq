use serde_json::to_string_pretty;
use serde::Serialize;
use clap::Parser;

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
    for lotto in &lottos {
        println!("lotto: {:?}", lotto);
    }

    util::divide();

    let lotto_1 = &lottos[0];
    match util::get_result().await {
        Err(err) => println!("fetch failed: {err}"),
        Ok(ssq_result_arr) => {
            for ssq_result in &ssq_result_arr {
                let lotto_result = lotto::LottoResult::new(ssq_result);
                lotto_result.calc(lotto_1);
            }
        }
    }
}
