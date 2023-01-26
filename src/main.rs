mod texas;
mod win_rate;
use clap::Parser;
use texas_win_rate::{texas::Card, win_rate::Stage};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    cards: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let cards: Vec<Card> = cli.cards.iter().map(|x| Card::from(x.as_str())).collect();
    let stage = Stage::new([cards[0], cards[1]], &cards[2..]);
    dbg!(&stage);
    dbg!(stage.win_rate());
}
