use heapless::Vec;
use std::fmt::{Debug, Display};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    /// 高牌
    HighCard,
    /// 一对
    Pair,
    /// 两对
    TwoPair,
    /// 三条
    ThreeOfAKind,
    /// 顺子
    Straight,
    /// 同花
    Flush,
    /// 葫芦
    FullHouse,
    /// 四条
    FourOfAKind,
    /// 同花顺
    StraightFlush,
    /// 皇家同花顺
    RoyalFlush,
}

impl HandType {
    /// cards should be sorted by number
    fn is_royal_flush(cards: &[Card]) -> bool {
        Self::is_strait_flush(cards) && cards[0].num == CardNum::Ten
    }

    /// cards should be sorted by number
    fn is_strait_flush(cards: &[Card]) -> bool {
        debug_assert_eq!(cards.len(), 5);
        Self::is_strait(cards) && Self::is_flush(cards)
    }

    fn is_flush(cards: &[Card]) -> bool {
        debug_assert_eq!(cards.len(), 5);
        cards[0].suit == cards[1].suit
            && cards[1].suit == cards[2].suit
            && cards[2].suit == cards[3].suit
            && cards[3].suit == cards[4].suit
    }

    /// cards should be sorted by number
    fn is_strait(cards: &[Card]) -> bool {
        debug_assert_eq!(cards.len(), 5);
        let mut success = true;
        let mut last = cards[0];
        for card in cards.iter().skip(1) {
            if !last.num.is_next(&card.num) {
                success = false;
            }

            last = *card;
        }

        if !success && cards[4].num == CardNum::Ace && cards[0].num == CardNum::Two {
            success = true;
            let mut last = cards[0];
            for card in cards.iter().skip(1).take(3) {
                if !last.num.is_next(&card.num) {
                    success = false;
                }

                last = *card;
            }
        }

        success
    }

    /// cards should be sorted by number
    #[allow(clippy::nonminimal_bool)]
    fn is_full_house(cards: &[Card]) -> bool {
        debug_assert_eq!(cards.len(), 5);
        (cards[0].num == cards[1].num
            && cards[1].num == cards[2].num
            && cards[3].num == cards[4].num)
            || (cards[0].num == cards[1].num
                && cards[2].num == cards[3].num
                && cards[3].num == cards[4].num)
    }

    fn four_of_a_kind(cards: &[Card]) -> Option<Vec<CardNum, 5>> {
        debug_assert_eq!(cards.len(), 5);
        if cards[0].num == cards[1].num
            && cards[1].num == cards[2].num
            && cards[2].num == cards[3].num
        {
            let mut vec = Vec::new();
            vec.extend([cards[0].num, cards[4].num]);
            return Some(vec);
        }

        if cards[3].num == cards[4].num
            && cards[1].num == cards[2].num
            && cards[2].num == cards[3].num
        {
            let mut vec = Vec::new();
            vec.extend([cards[4].num, cards[0].num]);
            return Some(vec);
        }

        None
    }

    fn three_of_a_kind(cards: &[Card]) -> Option<Vec<CardNum, 5>> {
        debug_assert_eq!(cards.len(), 5);
        let e01 = cards[0].num == cards[1].num;
        let e12 = cards[1].num == cards[2].num;
        let e23 = cards[2].num == cards[3].num;
        let e34 = cards[3].num == cards[4].num;
        #[allow(clippy::nonminimal_bool)]
        if (e01 && e12) || (e12 && e23) || (e23 && e34) {
            let mut ans = Vec::new();
            for card in cards {
                if card.num != cards[2].num {
                    ans.push(card.num).unwrap();
                }
            }

            ans.sort();
            ans.reverse();
            ans.insert(0, cards[2].num).unwrap();
            Some(ans)
        } else {
            None
        }
    }

    fn two_pair(cards: &[Card]) -> Option<Vec<CardNum, 5>> {
        debug_assert_eq!(cards.len(), 5);
        if cards[0].num == cards[1].num && cards[2].num == cards[3].num {
            let mut vec = Vec::new();
            vec.extend([cards[0].num, cards[2].num]);
            vec.sort_by_key(|x| -isize::from(*x));
            vec.push(cards[4].num).unwrap();
            return Some(vec);
        }
        if cards[0].num == cards[1].num && cards[3].num == cards[4].num {
            let mut vec = Vec::new();
            vec.extend([cards[0].num, cards[3].num]);
            vec.sort_by_key(|x| -isize::from(*x));
            vec.push(cards[2].num).unwrap();
            return Some(vec);
        }
        if cards[1].num == cards[2].num && cards[3].num == cards[4].num {
            let mut vec = Vec::new();
            vec.extend([cards[1].num, cards[3].num]);
            vec.sort_by_key(|x| -isize::from(*x));
            vec.push(cards[0].num).unwrap();
            return Some(vec);
        }

        None
    }

    fn pair(cards: &[Card]) -> Option<Vec<CardNum, 5>> {
        debug_assert_eq!(cards.len(), 5);
        if cards[0].num == cards[1].num {
            let mut vec = Vec::new();
            vec.extend([cards[0].num, cards[4].num, cards[3].num, cards[2].num]);
            return Some(vec);
        }

        if cards[1].num == cards[2].num {
            let mut vec = Vec::new();
            vec.extend([cards[1].num, cards[4].num, cards[3].num, cards[0].num]);
            return Some(vec);
        }

        if cards[2].num == cards[3].num {
            let mut vec = Vec::new();
            vec.extend([cards[2].num, cards[4].num, cards[1].num, cards[0].num]);
            return Some(vec);
        }

        if cards[3].num == cards[4].num {
            let mut vec = Vec::new();
            vec.extend([cards[3].num, cards[2].num, cards[1].num, cards[0].num]);
            return Some(vec);
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CardNum {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardNum {
    pub fn is_next(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (CardNum::Ace, CardNum::Two)
                | (CardNum::Two, CardNum::Three)
                | (CardNum::Three, CardNum::Four)
                | (CardNum::Four, CardNum::Five)
                | (CardNum::Five, CardNum::Six)
                | (CardNum::Six, CardNum::Seven)
                | (CardNum::Seven, CardNum::Eight)
                | (CardNum::Eight, CardNum::Nine)
                | (CardNum::Nine, CardNum::Ten)
                | (CardNum::Ten, CardNum::Jack)
                | (CardNum::Jack, CardNum::Queen)
                | (CardNum::Queen, CardNum::King)
                | (CardNum::King, CardNum::Ace)
        )
    }
}

impl From<usize> for CardNum {
    fn from(value: usize) -> Self {
        match value {
            1 => CardNum::Ace,
            2 => CardNum::Two,
            3 => CardNum::Three,
            4 => CardNum::Four,
            5 => CardNum::Five,
            6 => CardNum::Six,
            7 => CardNum::Seven,
            8 => CardNum::Eight,
            9 => CardNum::Nine,
            10 => CardNum::Ten,
            11 => CardNum::Jack,
            12 => CardNum::Queen,
            13 => CardNum::King,
            x => panic!("Invalid card type {x}"),
        }
    }
}

impl From<&str> for CardNum {
    fn from(value: &str) -> Self {
        match value {
            "1" | "A" | "a" => CardNum::Ace,
            "2" => CardNum::Two,
            "3" => CardNum::Three,
            "4" => CardNum::Four,
            "5" => CardNum::Five,
            "6" => CardNum::Six,
            "7" => CardNum::Seven,
            "8" => CardNum::Eight,
            "9" => CardNum::Nine,
            "10" | "T" | "t" => CardNum::Ten,
            "11" | "J" | "j" => CardNum::Jack,
            "12" | "Q" | "q" => CardNum::Queen,
            "13" | "K" | "k" => CardNum::King,
            x => panic!("Invalid card type {x}"),
        }
    }
}

impl From<CardNum> for usize {
    fn from(value: CardNum) -> Self {
        match value {
            CardNum::Ace => 1,
            CardNum::Two => 2,
            CardNum::Three => 3,
            CardNum::Four => 4,
            CardNum::Five => 5,
            CardNum::Six => 6,
            CardNum::Seven => 7,
            CardNum::Eight => 8,
            CardNum::Nine => 9,
            CardNum::Ten => 10,
            CardNum::Jack => 11,
            CardNum::Queen => 12,
            CardNum::King => 13,
        }
    }
}

impl From<CardNum> for isize {
    fn from(value: CardNum) -> Self {
        usize::from(value) as isize
    }
}

impl From<usize> for Suit {
    fn from(value: usize) -> Self {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            4 => Suit::Spade,
            _ => panic!("Invalid suit {value}"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Heart => f.write_str("♥️"),
            Suit::Diamond => f.write_str("♦️"),
            Suit::Club => f.write_str("♣️"),
            Suit::Spade => f.write_str("♠️"),
        }
    }
}

impl Display for CardNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardNum::Ace => f.write_str("A"),
            CardNum::Two => f.write_str("2"),
            CardNum::Three => f.write_str("3"),
            CardNum::Four => f.write_str("4"),
            CardNum::Five => f.write_str("5"),
            CardNum::Six => f.write_str("6"),
            CardNum::Seven => f.write_str("7"),
            CardNum::Eight => f.write_str("8"),
            CardNum::Nine => f.write_str("9"),
            CardNum::Ten => f.write_str("10"),
            CardNum::Jack => f.write_str("J"),
            CardNum::Queen => f.write_str("Q"),
            CardNum::King => f.write_str("K"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub num: CardNum,
}

#[derive(Debug)]
pub struct Hand {
    hand: HandType,
    hand_cmp_cards: Vec<CardNum, 5>,
    cards: Vec<Card, 5>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.hand_cmp_cards == other.hand_cmp_cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand.partial_cmp(&other.hand) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        debug_assert_eq!(self.hand_cmp_cards.len(), other.hand_cmp_cards.len());
        Some(self.hand_cmp_cards.cmp(&other.hand_cmp_cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    #[inline(always)]
    pub fn hand_type(&self) -> HandType {
        self.hand
    }

    #[inline(always)]
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    #[inline(always)]
    pub(crate) fn cmp_cards(&self) -> &[CardNum] {
        &self.hand_cmp_cards
    }
}

pub fn calc_hand(cards: &[Card]) -> Hand {
    let mut cards = Vec::from_slice(cards).unwrap();
    cards.sort_by_key(|x| x.num);
    if HandType::is_royal_flush(&cards) {
        Hand {
            hand: HandType::RoyalFlush,
            hand_cmp_cards: Vec::new(),
            cards,
        }
    } else if HandType::is_strait_flush(&cards) {
        cards.reverse();
        Hand {
            hand: HandType::StraightFlush,
            hand_cmp_cards: cards.iter().map(|x| x.num).collect(),
            cards,
        }
    } else if let Some(cmp) = HandType::four_of_a_kind(&cards) {
        Hand {
            hand: HandType::FourOfAKind,
            hand_cmp_cards: cmp,
            cards,
        }
    } else if HandType::is_full_house(&cards) {
        cards.reverse();
        Hand {
            hand: HandType::FullHouse,
            hand_cmp_cards: cards.iter().map(|x| x.num).collect(),
            cards,
        }
    } else if HandType::is_flush(&cards) {
        Hand {
            hand: HandType::Flush,
            hand_cmp_cards: cards.iter().map(|x| x.num).collect(),
            cards,
        }
    } else if HandType::is_strait(&cards) {
        cards.reverse();
        Hand {
            hand: HandType::Straight,
            hand_cmp_cards: cards.iter().map(|x| x.num).collect(),
            cards,
        }
    } else if let Some(cmp) = HandType::three_of_a_kind(&cards) {
        Hand {
            hand: HandType::ThreeOfAKind,
            hand_cmp_cards: cmp,
            cards,
        }
    } else if let Some(cmp) = HandType::two_pair(&cards) {
        Hand {
            hand: HandType::TwoPair,
            hand_cmp_cards: cmp,
            cards,
        }
    } else if let Some(cmp) = HandType::pair(&cards) {
        Hand {
            hand: HandType::Pair,
            hand_cmp_cards: cmp,
            cards,
        }
    } else {
        cards.reverse();
        Hand {
            hand: HandType::HighCard,
            hand_cmp_cards: cards.iter().map(|x| x.num).collect(),
            cards,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.num)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.num)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.suit == other.suit
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.num.cmp(&other.num) {
            std::cmp::Ordering::Equal => {}
            x => return Some(x),
        }

        if self.suit == other.suit {
            return Some(std::cmp::Ordering::Equal);
        }

        None
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match &value[0..1] {
            "1" | "H" | "h" => Card {
                suit: Suit::Heart,
                num: CardNum::from(&value[1..]),
            },
            "2" | "D" | "d" => Card {
                suit: Suit::Diamond,
                num: CardNum::from(&value[1..]),
            },
            "3" | "C" | "c" => Card {
                suit: Suit::Club,
                num: CardNum::from(&value[1..]),
            },
            "4" | "S" | "s" => Card {
                suit: Suit::Spade,
                num: CardNum::from(&value[1..]),
            },
            _ => {
                panic!("Invalid card {value}")
            }
        }
    }
}

pub fn iter_all_cards() -> impl Iterator<Item = Card> {
    let mut suit = 1;
    let mut num = 1;
    std::iter::from_fn(move || {
        if suit == 5 {
            None
        } else {
            let card = Card {
                suit: Suit::from(suit),
                num: CardNum::from(num),
            };

            if num == 13 {
                suit += 1;
                num = 1;
            } else {
                num += 1;
            }
            Some(card)
        }
    })
}

#[cfg(test)]
mod test {
    use crate::texas::{calc_hand, Card, CardNum, HandType, Suit};

    use super::iter_all_cards;

    #[test]
    fn test_ord() {
        assert!(CardNum::Ace > CardNum::King);
        assert!(CardNum::King > CardNum::Two);
        assert!(HandType::RoyalFlush > HandType::HighCard);
    }

    #[test]
    fn test_card() {
        let card: Card = "210".into();
        assert_eq!(card.suit, Suit::Diamond);
        assert_eq!(card.num, 10.into());
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(
            calc_hand(&[
                "21".into(),
                "32".into(),
                "43".into(),
                "19".into(),
                "2K".into()
            ])
            .hand,
            HandType::HighCard
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "31".into(),
                "43".into(),
                "19".into(),
                "2K".into()
            ])
            .hand,
            HandType::Pair
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "31".into(),
                "43".into(),
                "13".into(),
                "2K".into()
            ])
            .hand,
            HandType::TwoPair
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "31".into(),
                "41".into(),
                "13".into(),
                "2K".into()
            ])
            .hand,
            HandType::ThreeOfAKind
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "31".into(),
                "41".into(),
                "11".into(),
                "2K".into()
            ])
            .hand,
            HandType::FourOfAKind
        );

        assert_eq!(
            calc_hand(&[
                "2K".into(),
                "32".into(),
                "43".into(),
                "14".into(),
                "21".into()
            ])
            .hand,
            HandType::HighCard
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "3K".into(),
                "4Q".into(),
                "1J".into(),
                "2T".into()
            ])
            .hand,
            HandType::Straight
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "32".into(),
                "43".into(),
                "14".into(),
                "25".into()
            ])
            .hand,
            HandType::Straight
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "29".into(),
                "23".into(),
                "24".into(),
                "25".into()
            ])
            .hand,
            HandType::Flush
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "31".into(),
                "21".into(),
                "42".into(),
                "22".into()
            ])
            .hand,
            HandType::FullHouse
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "11".into(),
                "31".into(),
                "41".into(),
                "25".into()
            ])
            .hand,
            HandType::FourOfAKind
        );

        assert_eq!(
            calc_hand(&[
                "21".into(),
                "22".into(),
                "24".into(),
                "23".into(),
                "25".into()
            ])
            .hand,
            HandType::StraightFlush
        );

        assert_eq!(
            calc_hand(&[
                "210".into(),
                "2J".into(),
                "2K".into(),
                "2Q".into(),
                "2A".into()
            ])
            .hand,
            HandType::RoyalFlush
        );
    }

    #[test]
    fn test_hand() {
        let cards = [
            "211".into(),
            "212".into(),
            "213".into(),
            "210".into(),
            "21".into(),
        ];
        let hand = calc_hand(&cards);
        assert_eq!(hand.hand, HandType::RoyalFlush);

        let cards = [
            "211".into(),
            "212".into(),
            "213".into(),
            "210".into(),
            "29".into(),
        ];
        let hand = calc_hand(&cards);
        assert_eq!(hand.hand, HandType::StraightFlush);

        let cards_a = [
            "211".into(),
            "311".into(),
            "411".into(),
            "110".into(),
            "29".into(),
        ];
        let cards_b = [
            "211".into(),
            "311".into(),
            "411".into(),
            "112".into(),
            "29".into(),
        ];
        assert!(calc_hand(&cards_a) < calc_hand(&cards_b));

        let cards_a = [
            "211".into(),
            "311".into(),
            "410".into(),
            "110".into(),
            "29".into(),
        ];
        let cards_b = [
            "211".into(),
            "311".into(),
            "412".into(),
            "112".into(),
            "29".into(),
        ];
        assert!(calc_hand(&cards_a) < calc_hand(&cards_b));

        let cards_a = [
            "211".into(),
            "311".into(),
            "410".into(),
            "110".into(),
            "29".into(),
        ];
        let cards_b = [
            "211".into(),
            "311".into(),
            "410".into(),
            "110".into(),
            "212".into(),
        ];
        assert!(calc_hand(&cards_a) < calc_hand(&cards_b));

        let cards_a = [
            "211".into(),
            "311".into(),
            "47".into(),
            "110".into(),
            "29".into(),
        ];
        let cards_b = [
            "211".into(),
            "311".into(),
            "42".into(),
            "212".into(),
            "110".into(),
        ];
        assert!(calc_hand(&cards_a) < calc_hand(&cards_b));

        let cards_a = [
            "211".into(),
            "311".into(),
            "212".into(),
            "411".into(),
            "111".into(),
        ];
        let cards_b = [
            "213".into(),
            "211".into(),
            "311".into(),
            "411".into(),
            "111".into(),
        ];
        assert!(calc_hand(&cards_a) < calc_hand(&cards_b));
    }

    #[test]
    fn test_iter_all_cards() {
        assert_eq!(iter_all_cards().count(), 52);
    }
}
