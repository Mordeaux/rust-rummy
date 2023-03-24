use serde::{Deserialize, Serialize};

use crate::parser::{Card, GameState, TurnPhaseEnum};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DrawPhaseMove {
    #[serde(rename = "draw_from_deck")]
    DrawFromDeck,
    #[serde(rename = "draw_from_discards")]
    DrawFromDiscards(Card),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlayPhaseMove {
    PlayRun(Vec<Card>),
    PlayGroup(Vec<Card>),
    Discard(Card),
}

#[derive(Serialize, Deserialize)]
pub enum GameMoves {
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "draw")]
    Draw(Vec<DrawPhaseMove>),
    #[serde(rename = "play")]
    Play(Vec<PlayPhaseMove>),
}

pub fn get_available_moves(game_state: GameState) -> GameMoves {
    match game_state.turn_phase {
        TurnPhaseEnum::WaitForTurn => GameMoves::Wait,
        TurnPhaseEnum::Draw => GameMoves::Draw(
            game_state
                .discards
                .iter()
                .map(|card| DrawPhaseMove::DrawFromDiscards(card.clone()))
                .chain(std::iter::once(DrawPhaseMove::DrawFromDeck))
                .collect::<Vec<DrawPhaseMove>>(),
        ),
        TurnPhaseEnum::Play => GameMoves::Play(Vec::new()),
    }
}
