use heapless::Vec as HeaplessVec;
use statistical::{mean, standard_deviation};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    iter,
};

use crate::texas::{calc_hand, iter_all_cards, Card, Hand, HandType};

#[derive(Debug)]
pub struct Stage {
    pub_cards: HeaplessVec<Card, 5>,
    my_cards: [Card; 2],
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hole_cards: {:?}, community_cards: {:?}",
            self.my_cards, self.pub_cards
        )
    }
}

#[derive(Debug, Clone)]
pub struct Odds {
    pub win: f64,
    pub tie: f64,
    pub hand_rate: BTreeMap<HandType, f64>,
}

impl Display for Odds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "win: {:.2}%, tie: {:.2}%",
            self.win * 100.0,
            self.tie * 100.0
        )
        .and_then(|_| writeln!(f, "hand_rate: {:#?}", self.hand_rate))
    }
}

#[derive(Debug, Clone)]
pub struct WinRate {
    pub mean: f64,
    pub mean_tie_rate: f64,
    pub min: f64,
    pub max: f64,
    pub percentile25: f64,
    pub median: f64,
    pub percentile75: f64,
    pub std: f64,
    pub self_rate: BTreeMap<HandType, f64>,
    pub other_rate: BTreeMap<HandType, f64>,
    pub diff_rate: BTreeMap<HandType, f64>,
}

impl Stage {
    pub fn new(my_cards: [Card; 2], pub_cards: &[Card]) -> Self {
        let pub_cards = HeaplessVec::from_slice(pub_cards).unwrap();
        assert!(
            (pub_cards.len() >= 3 && pub_cards.len() <= 5) || pub_cards.is_empty(),
            "Invalid pub_cards length {}",
            pub_cards.len()
        );
        Self {
            pub_cards,
            my_cards,
        }
    }

    pub fn win_rate(&self) -> WinRate {
        let (my_hands, all_hands) = self.enumerate_hands();
        let mut win_rates = Vec::with_capacity(my_hands.len());
        let mut tie_rates = Vec::with_capacity(my_hands.len());
        // dbg!(my_hands.len(), all_hands.len());
        // all_hands.iter().enumerate().for_each(|(i, x)| {
        //     println!("{}: {:?}", i, x);
        // });
        for hand in my_hands.iter() {
            let without_tie_rank = match all_hands.binary_search_by(|x| match x.cmp(hand) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                x => x,
            }) {
                Ok(i) => i,
                Err(i) => i,
            };
            let win_rate = without_tie_rank as f64 / all_hands.len() as f64;
            let with_tie_rank = match all_hands.binary_search_by(|x| match x.cmp(hand) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Less,
                x => x,
            }) {
                Ok(i) => i,
                Err(i) => i,
            };
            let tie_rate = (with_tie_rank - without_tie_rank) as f64 / all_hands.len() as f64;
            win_rates.push(win_rate);
            tie_rates.push(tie_rate);
        }
        win_rates.sort_unstable_by(f64::total_cmp);
        let self_rate = count_hand_type_freq(&my_hands);
        let other_rate = count_hand_type_freq(&all_hands);
        WinRate {
            mean: mean(&win_rates),
            mean_tie_rate: mean(&tie_rates),
            median: win_rates[win_rates.len() / 2],
            percentile25: win_rates[win_rates.len() / 4],
            percentile75: win_rates[win_rates.len() / 4 * 3],
            std: if win_rates.len() > 2 {
                standard_deviation(&win_rates, None)
            } else {
                0.
            },
            min: win_rates[0],
            max: win_rates[win_rates.len() - 1],
            diff_rate: self_rate
                .iter()
                .map(|(k, v)| {
                    let other = other_rate.get(k).unwrap_or(&0.);
                    (*k, v - other)
                })
                .chain(other_rate.iter().map(|(k, v)| {
                    let this = self_rate.get(k).unwrap_or(&0.);
                    (*k, this - v)
                }))
                .collect(),
            self_rate,
            other_rate,
        }
    }

    fn enumerate_hands(&self) -> (Vec<Hand>, Vec<Hand>) {
        let (my_hands, all_hands) = if self.pub_cards.is_empty() {
            // FIXME: this is not accurate
            let my_hands = fill_5_and_get_all_hands(&self.my_cards);
            let all_hands = fill_5_and_get_all_hands(&[]);
            (my_hands, all_hands)
        } else {
            let mut vec: HeaplessVec<Card, 7> = HeaplessVec::new();
            vec.extend_from_slice(&self.pub_cards).unwrap();
            vec.extend_from_slice(&self.my_cards).unwrap();
            let my_hands = fill_7_and_get_all_hands(&vec);
            let all_hands = fill_7_and_get_all_hands(&self.pub_cards);
            (my_hands, all_hands)
        };
        (my_hands, all_hands)
    }

    pub fn win_rate_with_n_players(&self, n: usize) -> Odds {
        assert!(n >= 2, "n_players must be >= 2");
        let (my_hands, all_hands) = self.enumerate_hands();
        let mut win_rates = Vec::with_capacity(my_hands.len());
        let mut tie_rates = Vec::with_capacity(my_hands.len());
        for hand in my_hands.iter() {
            let without_tie_rank = match all_hands.binary_search_by(|x| match x.cmp(hand) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                x => x,
            }) {
                Ok(i) => i,
                Err(i) => i,
            };
            let win_rate = without_tie_rank as f64 / all_hands.len() as f64;
            let with_tie_rank = match all_hands.binary_search_by(|x| match x.cmp(hand) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Less,
                x => x,
            }) {
                Ok(i) => i,
                Err(i) => i,
            };
            let tie_rate = (with_tie_rank - without_tie_rank) as f64 / all_hands.len() as f64;
            let win_or_tie = win_rate + tie_rate;

            let lose_rate_with_n_players = 1. - win_or_tie.powi(n as i32);
            let win_rate_with_n_players = win_rate.powi(n as i32);
            let tie_rate_with_n_players = 1. - lose_rate_with_n_players - win_rate_with_n_players;
            win_rates.push(win_rate_with_n_players);
            tie_rates.push(tie_rate_with_n_players);
        }
        Odds {
            win: mean(&win_rates),
            tie: mean(&tie_rates),
            hand_rate: count_hand_type_freq(&my_hands),
        }
    }
}

fn count_hand_type_freq(hands: &[Hand]) -> BTreeMap<HandType, f64> {
    let mut map = BTreeMap::new();
    for hand in hands {
        let count = map.entry(hand.hand_type()).or_insert(0.);
        *count += 1.;
    }

    for (_, value) in map.iter_mut() {
        *value /= hands.len() as f64;
    }

    map
}

fn get_max_hand(origin_cards: &[Card]) -> Hand {
    assert_eq!(origin_cards.len(), 7);
    let mut max_hand = None;
    for i in 0..origin_cards.len() - 1 {
        for j in i + 1..origin_cards.len() {
            let mut cards: HeaplessVec<Card, 7> = HeaplessVec::new();
            cards.extend_from_slice(origin_cards).unwrap();
            cards.remove(j);
            cards.remove(i);
            let hand = calc_hand(&cards);
            if let Some(max_hand) = &mut max_hand {
                if hand > *max_hand {
                    *max_hand = hand;
                }
            } else {
                max_hand = Some(hand);
            }
        }
    }

    max_hand.unwrap()
}

fn fill_5_and_get_all_hands(cards: &[Card]) -> Vec<Hand> {
    assert!(cards.len() <= 5);
    let mut ans: Vec<Hand> = enumerate_n_cards(cards, 5 - cards.len())
        .map(|x| calc_hand(&x))
        .collect();
    ans.sort_unstable();
    ans
}

fn fill_7_and_get_all_hands(cards: &[Card]) -> Vec<Hand> {
    assert!(cards.len() >= 3);
    let mut ans: Vec<Hand> = enumerate_n_cards(cards, 7 - cards.len())
        .map(|x| get_max_hand(&x))
        .collect();
    ans.sort_unstable();
    ans
}

fn enumerate_n_cards(cards: &[Card], n: usize) -> impl Iterator<Item = HeaplessVec<Card, 7>> {
    let mut iter_stacks: HeaplessVec<usize, 7> = HeaplessVec::new();
    let mut all_cards: HeaplessVec<Card, 52> = HeaplessVec::new();
    for card in iter_all_cards().filter(|x| !cards.contains(x)) {
        all_cards.push(card).unwrap();
    }
    for i in 0..n {
        iter_stacks.push(i).unwrap();
    }

    let cards: Vec<_> = cards.to_vec();
    let mut empty_returned = false;
    std::iter::from_fn(move || {
        if iter_stacks.is_empty() {
            if empty_returned {
                return None;
            } else {
                empty_returned = true;
                let mut result: HeaplessVec<Card, 7> = HeaplessVec::new();
                result.extend_from_slice(&cards).unwrap();
                return Some(result);
            }
        }

        if iter_stacks[0] == all_cards.len() {
            return None;
        }

        let mut result: HeaplessVec<Card, 7> = HeaplessVec::new();
        let mut inc_index = n - 1;
        for i in 0..n {
            let index = iter_stacks[i];
            result.push(all_cards[index]).unwrap();
        }

        let origin_inc_index = inc_index;
        loop {
            if iter_stacks[inc_index] >= all_cards.len() - n + inc_index {
                if inc_index == 0 {
                    iter_stacks[inc_index] = all_cards.len();
                    break;
                } else {
                    inc_index -= 1;
                }
            } else {
                iter_stacks[inc_index] += 1;
                break;
            }
        }
        for i in inc_index + 1..=origin_inc_index {
            if i == 0 {
                continue;
            }
            iter_stacks[i] = iter_stacks[i - 1] + 1;
        }

        result.extend_from_slice(&cards).unwrap();
        Some(result)
    })
}

#[cfg(feature = "wasm")]
mod wasm {}

#[cfg(test)]
mod test {
    use super::{enumerate_n_cards, get_max_hand, Stage};
    use crate::{odds::fill_7_and_get_all_hands, texas::HandType};

    #[test]
    fn test_max_hand() {
        let hand = get_max_hand(&[
            "41".into(),
            "28".into(),
            "28".into(),
            "28".into(),
            "11".into(),
            "21".into(),
            "31".into(),
        ]);
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);
        assert_eq!(hand.cmp_cards().to_vec(), vec!["1".into(), "8".into()]);
    }

    #[test]
    fn test_append_cards() {
        dbg!(fill_7_and_get_all_hands(&[
            "41".into(),
            "31".into(),
            "21".into(),
            "11".into(),
            "18".into(),
        ],));
    }

    #[test]
    fn win_rate() {
        let stage = Stage::new(["45".into(), "H6".into()], &[]);
        dbg!(stage.win_rate());
    }

    #[test]
    fn win_rate_2() {
        let stage = Stage::new(
            ["28".into(), "13".into()],
            &["310".into(), "210".into(), "38".into()],
        );
        dbg!(stage.win_rate());
    }

    #[test]
    fn win_rate_3() {
        let stage = Stage::new(
            ["34".into(), "45".into()],
            &[
                "2K".into(),
                "12".into(),
                "23".into(),
                "17".into(),
                "19".into(),
            ],
        );
        dbg!(stage.win_rate_with_n_players(5));
    }

    #[test]
    fn should_i_wait_for_flush() {
        let hands = fill_7_and_get_all_hands(&[
            "41".into(),
            "48".into(),
            "49".into(),
            "44".into(),
            "21".into(),
        ]);
        let flush_count = hands
            .iter()
            .filter(|x| x.hand_type() == HandType::Flush)
            .count();
        println!("{}", flush_count as f64 / hands.len() as f64);
    }

    #[test]
    fn test_append() {
        let cards = enumerate_n_cards(&[], 5);
        assert_eq!(cards.count(), 2598960);
        let cards = enumerate_n_cards(&[], 4);
        assert_eq!(cards.count(), 52 * 51 * 50 * 49 / 4 / 3 / 2);
    }
}
