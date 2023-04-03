use core::panic;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Card {
    rank: usize,
    suit: String,
    #[serde(default = "Card::true_default")]
    playable: bool,
}

impl Card {
    pub fn new(rank: usize, suit: String) -> Card {
        Card {
            rank,
            suit,
            playable: true,
        }
    }

    pub fn is_playable(&self) -> bool {
        self.playable
    }

    pub fn set_not_playable(&mut self) {
        self.playable = false;
    }

    pub fn set_playable(&mut self) {
        self.playable = true;
    }

    fn true_default() -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "card_type", content = "card")]
pub enum CardOption {
    #[serde(rename = "visible")]
    Visible(Card),
    #[serde(rename = "hidden")]
    Hidden,
}

impl CardOption {
    pub fn get_card(&self) -> &Card {
        match self {
            Self::Visible(card) => card,
            Self::Hidden => panic!("Cannot view opponents' cards!"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TurnPhaseEnum {
    #[serde(rename = "WAIT_FOR_TURN")]
    WaitForTurn,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "PLAY")]
    Play,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    id: usize,
    score: usize,
    pub order_index: usize,
    pub hand: Vec<CardOption>,
    melds: Vec<Vec<CardOption>>,
    pub username: String,
}

impl Player {
    pub fn set_hand(&mut self, new_hand: Vec<CardOption>) {
        self.hand = new_hand;
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "move_type")]
pub enum DrawPhaseMove {
    #[serde(rename = "draw_from_deck")]
    DrawFromDeck(CardOption),
    #[serde(rename = "draw_from_discards")]
    DrawFromDiscards(CardOption),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "move_type")]
pub enum PlayPhaseMove {
    #[serde(rename = "play_run")]
    PlayRun(Vec<CardOption>),
    #[serde(rename = "play_group")]
    PlayGroup(Vec<CardOption>),
    #[serde(rename = "discard")]
    Discard(CardOption),
}

// Moves are stored in a vector to facilitate
// serializing into json.
#[derive(Serialize, Deserialize, Clone)]
pub enum GameMoves {
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "draw")]
    Draw(Vec<DrawPhaseMove>),
    #[serde(rename = "play")]
    Play(Vec<PlayPhaseMove>),
}

impl GameMoves {
    pub fn new_from_json_str(game_moves_json: &str) -> GameMoves {
        serde_json::from_str::<GameMoves>(game_moves_json).unwrap()
    }
}

#[cfg(test)]
mod gameplay_tests {
    use super::*;

    fn test_card_equality() {
        assert_eq!(
            CardOption::Visible(Card {
                suit: "diamonds".into(),
                rank: 10,
                playable: true,
            }),
            CardOption::Visible(Card {
                suit: "diamonds".into(),
                rank: 10,
                playable: true,
            })
        );
    }
}
