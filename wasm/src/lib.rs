mod utils;

use js_sys::{Object, Reflect};
use texas_odds::{odds::Odds as OriginOdds, odds::Stage as OriginStage, texas::HandType};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Odds {
    win: f64,
    tie: f64,
    hand_type_rates: [f64; 10],
}

impl From<OriginOdds> for Odds {
    fn from(value: OriginOdds) -> Self {
        let mut hand_type_rates = [0.; 10];
        for (i, hand) in HandType::ALL.iter().enumerate() {
            let rate = value.hand_rate.get(hand).copied().unwrap_or(0.0);
            hand_type_rates[i] = rate;
        }

        Self {
            win: value.win,
            tie: value.tie,
            hand_type_rates,
        }
    }
}

#[wasm_bindgen]
pub struct Stage {
    stage: OriginStage,
    n_players: usize,
    odds: Option<Odds>,
}

#[wasm_bindgen]
impl Odds {
    #[wasm_bindgen(getter)]
    pub fn win(&self) -> f64 {
        self.win
    }

    #[wasm_bindgen(getter)]
    pub fn tie(&self) -> f64 {
        self.tie
    }

    pub fn hand_type_rates(&self) -> Object {
        let obj = Object::new();
        for (i, hand) in HandType::ALL.iter().enumerate() {
            Reflect::set(
                &obj,
                &hand.to_string().into(),
                &JsValue::from_f64(self.hand_type_rates[i]),
            )
            .unwrap();
        }
        obj
    }
}

#[wasm_bindgen]
impl Stage {
    #[wasm_bindgen(constructor)]
    pub fn new(
        n_players: usize,
        hole_card_0: &str,
        hole_card_1: &str,
        community_card: &str,
    ) -> Self {
        Self {
            n_players,
            stage: OriginStage::new(
                [hole_card_0.into(), hole_card_1.into()],
                &community_card
                    .split_whitespace()
                    .map(|x| x.into())
                    .collect::<Vec<_>>(),
            ),
            odds: None,
        }
    }

    pub fn odds(&mut self) -> Odds {
        if let Some(x) = self.odds {
            return x;
        }

        let odds = self.stage.win_rate_with_n_players(self.n_players);
        self.odds = Some(odds.into());
        self.odds.unwrap()
    }
}
