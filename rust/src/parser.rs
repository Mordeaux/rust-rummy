use core::panic;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Card {
    rank: usize,
    suit: String,
}

impl Card {
    pub fn new(rank: usize, suit: String) -> Card {
        Card { rank, suit }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "card_type", content = "card")]
pub enum CardOption {
    #[serde(rename = "own_card")]
    OwnCard(Card),
    #[serde(rename = "discard_card")]
    DiscardCard(Card),
    #[serde(rename = "meld_card")]
    MeldCard(Card),
    #[serde(rename = "opponent_card")]
    OpponentCard,
}

impl CardOption {
    pub fn get_card(&self) -> &Card {
        match self {
            Self::OwnCard(card) | Self::DiscardCard(card) | Self::MeldCard(card) => card,
            Self::OpponentCard => panic!("Cannot view opponents' cards!"),
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
    order_index: usize,
    pub hand: Vec<CardOption>,
    melds: Vec<Vec<CardOption>>,
    username: String,
}

impl Player {
    pub fn set_hand(&mut self, new_hand: Vec<CardOption>) {
        self.hand = new_hand;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "move_type")]
pub enum DrawPhaseMove {
    #[serde(rename = "draw_from_deck")]
    DrawFromDeck,
    #[serde(rename = "draw_from_discards")]
    DrawFromDiscards(CardOption),
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub enum GameMoves {
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "draw")]
    Draw(Vec<DrawPhaseMove>),
    #[serde(rename = "play")]
    Play(Vec<PlayPhaseMove>),
}

#[cfg(test)]
mod gameplay_tests {
    use super::*;

    fn test_card_equality() {
        assert_eq!(
            CardOption::OwnCard(Card {
                suit: "diamonds".into(),
                rank: 10
            }),
            CardOption::OwnCard(Card {
                suit: "diamonds".into(),
                rank: 10
            })
        );
    }
}
