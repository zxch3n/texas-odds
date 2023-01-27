<div align="center">
  <h1>Texas Hold'em Odds Calculator</h1>
  <h3> 德州扑克胜率计算器 </h3>
</div>

[**Try it online**](https://texas-odds-aah9.vercel.app/)

(A hobby project just for fun 🦀)

## CLI Usage

> Currently you need to install rust and clone this project to run the code

```
Texas Hold'em odds calculator

Usage: texas-odds [OPTIONS] <HOLE_CARDS_0> <HOLE_CARDS_1> [COMMUNITY_CARDS]...

Arguments:
  <HOLE_CARDS_0>
          Your cards no.1  -- 手牌 1

  <HOLE_CARDS_1>
          Your cards no.2  -- 手牌 2

  [COMMUNITY_CARDS]...
          The community cards -- 公开池

          It should be empty or at least 3 cards.

          Input format:

          >  h2 d3 s4 c5 d13

          - hole_cards = [♥️2, ♦️3]

          - community_cards = [♠️4, ♣️5, ♦️K]

          h = hearts 红心 ♥️ , d = diamonds 方块 ♦️, s = spades 黑桃 ♠️, c = clubs 梅花 ♣️

Options:
  -n, --n-players <N_PLAYERS>
          The number of players (default = 2)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

**`cargo run -r HK DA`**

- my cards: [ ♥️K, ♦️A, ]
- public cards: []

**`cargo run -r HK DA H3 C4 S10`**

- my cards: [ ♥️K, ♦️A, ]
- public cards: [ ♥️3, ♣️4, ♠️10, ]

**`cargo run -r 1K 2A 33 44 12 13`**

- my cards: [ ♥️K, ♦️A, ],
- public cards: [ ♣️3, ♠️4, ♥️2, ♥️3, ],

**Example Output**

```log
> cargo run -r SK HA D10 CJ C12 -n 8

8 Players
hole_cards: [♠️K, ♥️A], community_cards: [♦️10, ♣️J, ♣️Q]
win: 36.74%, tie: 30.09%
hand_rate: {Straight: 1.0}
```

```log
> cargo run -r SK HA D10 CJ C8 -n 8

8 Players
hole_cards: [♠️K, ♥️A], community_cards: [♦️10, ♣️J, ♣️8]
win: 11.48%, tie: 1.29%
hand_rate: {
    HighCard: 0.2960222016651249,
    Pair: 0.42738205365402404,
    TwoPair: 0.08325624421831637,
    ThreeOfAKind: 0.013876040703052728,
    Straight: 0.17946345975948197,
}
```

```log
> cargo run -r S1 HK -n 3

3 Players
hole_cards: [♠️A, ♥️K], community_cards: []
win: 35.80%, tie: 0.03%
hand_rate: {
    HighCard: 0.5355102040816326,
    Pair: 0.40408163265306124,
    TwoPair: 0.04040816326530612,
    ThreeOfAKind: 0.015714285714285715,
    Straight: 0.0032653061224489797,
    FullHouse: 0.0009183673469387755,
    FourOfAKind: 0.00010204081632653062,
}
```
