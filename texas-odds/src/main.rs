mod odds;
mod sim;
mod texas;
use clap::Parser;
use odds::Stage;

#[derive(Parser)]
/// Texas Hold'em odds calculator
#[command(author, version)]
struct Cli {
    /// Your cards no.1  -- 手牌 1
    hole_cards_0: String,
    /// Your cards no.2  -- 手牌 2
    hole_cards_1: String,
    /// The community cards -- 公开池
    ///
    /// It should be empty or at least 3 cards.
    ///
    /// Input format:
    ///
    /// >  h2 d3 s4 c5 d13
    ///
    /// - hole_cards = [♥️2, ♦️3]
    ///
    /// - community_cards = [♠️4, ♣️5, ♦️K]
    ///
    /// h = hearts 红心 ♥️ , d = diamonds 方块 ♦️, s = spades 黑桃 ♠️, c = clubs 梅花 ♣️
    community_cards: Vec<String>,

    /// The number of players (default = 2)
    #[arg(short, long, value_name = "N_PLAYERS")]
    n_players: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let stage = Stage::new(
        [
            cli.hole_cards_0.as_str().into(),
            cli.hole_cards_1.as_str().into(),
        ],
        &cli.community_cards
            .iter()
            .map(|x| x.as_str().into())
            .collect::<Vec<_>>(),
    );
    let n = cli.n_players.unwrap_or(2);
    println!("{} Players", n);
    println!("{}", &stage);
    println!("{}", stage.win_rate_with_n_players(n));
}
