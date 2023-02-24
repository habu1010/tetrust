mod ai;
mod block;
mod game;
mod play;
mod ui;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arg {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    Normal,
    Auto,
}

fn main() {
    let arg = Arg::parse();
    let result = match arg.mode {
        None | Some(Mode::Normal) => play::normal(),
        Some(Mode::Auto) => play::auto(),
    };

    if let Err(err) = result {
        println!("Error: {}", err);
    }
}
