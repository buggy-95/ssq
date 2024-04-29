use serde_json::to_string_pretty;
use serde::Serialize;
use clap::Parser;
use lotto::Lotto;

mod lotto;
mod util;

#[derive(Serialize, Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

fn main() {
    let args = Args::parse();
    let args_json = to_string_pretty(&args).unwrap();
    println!("args: {}", args_json);

    let lotto = Lotto::new("01,02,03,04,05,06-07 2");
    println!("red: {:?}", lotto.red_arr);
    println!("blue: {:?}", lotto.blue_arr);
    println!("scale: {}", lotto.scale);
}
