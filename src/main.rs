use clap::Parser;

mod util;

#[derive(Parser, Debug)]
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
    let str = "01,02,03,04,05,06-07";
    let (red, blue, scale) = util::parse_lotto(str);
    println!("red: {:?}", red);
    println!("blue: {:?}", blue);
    println!("scale: {:?}", scale);

    let args = Args::parse();
    println!("args: {:?}", args);
}
