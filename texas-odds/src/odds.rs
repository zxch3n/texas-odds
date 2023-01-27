use heapless::Vec as HeaplessVec;
use statistical::{mean, standard_deviation};
use std::{collections::BTreeMap, fmt::Debug};

use crate::texas::{calc_hand, iter_all_cards, Card, Hand, HandType};

pub struct Stage {
    pub_cards: HeaplessVec<Card, 5>,
    my_cards: [Card; 2],
}

impl Debug for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stage")
            .field("pub_cards", &self.pub_cards)
            .field("my_cards", &self.my_cards)
            .finish()
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
        let (my_hands, all_hands) = if self.pub_cards.is_empty() {
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
        let self_rate = count_hand_type(&my_hands);
        let other_rate = count_hand_type(&all_hands);
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
}

fn count_hand_type(hands: &[Hand]) -> BTreeMap<HandType, f64> {
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
    let filled: Vec<HeaplessVec<Card, 7>> = append_n_cards(cards, 5 - cards.len());
    let mut ans: Vec<Hand> = filled.into_iter().map(|x| calc_hand(&x)).collect();
    ans.sort();
    ans
}

fn fill_7_and_get_all_hands(cards: &[Card]) -> Vec<Hand> {
    assert!(cards.len() >= 3);
    let filled: Vec<HeaplessVec<Card, 7>> = append_n_cards(cards, 7 - cards.len());
    let mut ans: Vec<Hand> = filled.into_iter().map(|x| get_max_hand(&x)).collect();
    ans.sort();
    ans
}

// TODO: use iterator
fn append_n_cards(cards: &[Card], n: usize) -> Vec<HeaplessVec<Card, 7>> {
    let mut result: Vec<HeaplessVec<Card, 7>> = Vec::new();
    result.push(HeaplessVec::new());
    for _ in 0..n {
        let cur_result = std::mem::take(&mut result);
        for each in cur_result {
            if let Some(last) = each.last() {
                for card in iter_all_cards()
                    .skip_while(|x| x != last)
                    .filter(|x| !cards.contains(x) && !each.contains(x))
                {
                    let mut new_cards = each.clone();
                    new_cards.push(card).unwrap();
                    result.push(new_cards);
                }
            } else {
                for card in iter_all_cards().filter(|x| !cards.contains(x)) {
                    let mut new_cards = each.clone();
                    new_cards.push(card).unwrap();
                    result.push(new_cards);
                }
            }
        }
    }

    for each in result.iter_mut() {
        each.extend_from_slice(cards).unwrap();
    }

    result
}

#[cfg(test)]
mod test {
    use super::{append_n_cards, get_max_hand, Stage};
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
        let stage = Stage::new(
            ["45".into(), "H6".into()],
            &[
                "410".into(),
                "24".into(),
                "4J".into(),
                "15".into(),
                "2J".into(),
            ],
        );
        dbg!(stage.win_rate());
    }

    #[test]
    fn win_rate_2() {
        let stage = Stage::new(
            ["28".into(), "13".into()],
            &[
                "310".into(),
                "210".into(),
                "38".into(),
                "2J".into(),
                "39".into(),
            ],
        );
        dbg!(stage.win_rate());
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
        let cards = append_n_cards(&[], 5);
        println!("{}", cards.len());
    }
}
