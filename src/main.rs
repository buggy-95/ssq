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

fn main() {
    let args = Args::parse();
    let args_json = to_string_pretty(&args).unwrap();
    println!("args: {}", args_json);

    util::divide();

    let lottos = util::get_inputs(&args);
    for lotto in lottos {
        println!("lotto: {:?}", lotto);
    }
}
