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
    for lotto in lottos {
        println!("lotto: {:?}", lotto);
    }

    util::divide();

    match util::get_result().await {
        Ok(_) => println!("fetch success"),
        Err(err) => println!("fetch failed: {err}"),
    }

    util::calc_result(
        &lotto::Lotto::new("01,02,03,04,05,06-07"),
        &lotto::Lotto::new("01,02,03,04,05,06-07"),
    );
}
