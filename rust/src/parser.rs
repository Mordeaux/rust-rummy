use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Card {
    rank: usize,
    suit: String,
}

#[derive(Serialize, Deserialize)]
struct OpponentCard;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "card")]
enum CardOption {
    OwnCard(Card),
    OpponentCard,
}

#[derive(Serialize, Deserialize)]
enum TurnPhaseEnum {
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
    melds: Vec<Vec<Card>>,
    username: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    id: usize,
    players: Vec<Player>,
    current_turn: usize,
    turn_phase: TurnPhaseEnum,
    discards: Vec<Card>,
    name: String,
}

pub fn common_parse_game_state(game_state_json: &str) -> GameState {
    serde_json::from_str::<GameState>(game_state_json).unwrap()
}
