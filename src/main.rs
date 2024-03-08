use clap::Parser;

use crate::responses::json_response_single;
mod responses;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    option: String,
}

fn main() {
    let args = Args::parse();
    if args.option == "single-json" {
        json_response_single("E501".to_string());
    }
    println!("option: {:?}",args.option);
}
