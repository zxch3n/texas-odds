# Texas Hold'em Winning Percentage Calculator

(A hobby project just for fun 🦀

## Usage

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
> cargo run -r 13 36

[src/main.rs:17] &stage = Stage {
    pub_cards: [],
    my_cards: [
        ♥️3,
        ♣️6,
    ],
}
[src/main.rs:18] stage.win_rate() = WinRate {
    mean: 0.41689790458960785,
    min: 0.00048827223196971095,
    max: 0.9998210822790655,
    percentile25: 0.1484378366731308,
    median: 0.42578146643272696,
    percentile75: 0.6374515190691661,
    std: 0.2849603385083306,
    self_rate: {
        HighCard: 0.5322448979591837,
        Pair: 0.40408163265306124,
        TwoPair: 0.04040816326530612,
        ThreeOfAKind: 0.015714285714285715,
        Straight: 0.006530612244897959,
        FullHouse: 0.0009183673469387755,
        FourOfAKind: 0.00010204081632653062,
    },
    other_rate: {
        HighCard: 0.5011773940345369,
        Pair: 0.4225690276110444,
        TwoPair: 0.0475390156062425,
        ThreeOfAKind: 0.02112845138055222,
        Straight: 0.003924646781789639,
        Flush: 0.001965401545233478,
        FullHouse: 0.0014405762304921968,
        FourOfAKind: 0.00024009603841536616,
        StraightFlush: 1.3851694523963431e-5,
        RoyalFlush: 1.5390771693292702e-6,
    },
    diff_rate: {
        HighCard: 0.031067503924646855,
        Pair: -0.018487394957983183,
        TwoPair: -0.007130852340936376,
        ThreeOfAKind: -0.005414165666266506,
        Straight: 0.002605965463108321,
        Flush: -0.001965401545233478,
        FullHouse: -0.0005222088835534213,
        FourOfAKind: -0.00013805522208883554,
        StraightFlush: -1.3851694523963431e-5,
        RoyalFlush: -1.5390771693292702e-6,
    },
}
```
