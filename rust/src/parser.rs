use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    rank: usize,
    suit: String,
}

#[derive(Serialize, Deserialize)]
struct OpponentCard;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize)]
pub enum TurnPhaseEnum {
    #[serde(rename = "WAIT_FOR_TURN")]
    WaitForTurn,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "PLAY")]
    Play,
}

#[derive(Serialize, Deserialize)]
struct Player {
    id: usize,
    score: usize,
    order_index: usize,
    hand: Vec<CardOption>,
    melds: Vec<Vec<CardOption>>,
    username: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    id: usize,
    players: Vec<Player>,
    current_turn: usize,
    pub turn_phase: TurnPhaseEnum,
    pub discards: Vec<CardOption>,
    name: String,
}

pub fn common_parse_game_state(game_state_json: &str) -> GameState {
    serde_json::from_str::<GameState>(game_state_json).unwrap()
}
