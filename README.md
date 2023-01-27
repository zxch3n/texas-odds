<div align="center">
  <h1>Texas Hold'em Odds Calculator</h1>
  <h3> 德州扑克胜率计算器 </h3>
</div>

Currently it only calculate the winning rate when there is only one opponent.

(A hobby project just for fun 🦀)

## Usage

> Currently you need to install rust and clone this project to run the code

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
> cargo run -r 1K 2A 23 34 35 36 37

[src/main.rs:17] &stage = Stage {
    pub_cards: [
        ♦️3,
        ♣️4,
        ♣️5,
        ♣️6,
        ♣️7,
    ],
    my_cards: [
        ♥️K,
        ♦️A,
    ],
}
[src/main.rs:18] stage.win_rate() = WinRate {
    mean: 0.0,
    mean_tie_rate: 0.5420906567992599,
    min: 0.0,
    max: 0.0,
    percentile25: 0.0,
    median: 0.0,
    percentile75: 0.0,
    std: 0.0,
    self_rate: {
        Straight: 1.0,
    },
    other_rate: {
        Straight: 0.6503237742830712,
        Flush: 0.26549491211840887,
        StraightFlush: 0.0841813135985199,
    },
    diff_rate: {
        Straight: 0.3496762257169288,
        Flush: -0.26549491211840887,
        StraightFlush: -0.0841813135985199,
    },
}
```
