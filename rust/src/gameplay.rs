use serde::{Deserialize, Serialize};

use crate::parser::{CardOption, DrawPhaseMove, GameMoves, Player, TurnPhaseEnum};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    id: usize,
    players: Vec<Player>,
    current_turn: usize,
    turn_phase: TurnPhaseEnum,
    discards: Vec<CardOption>,
    name: String,
}

impl GameState {
    pub fn new_from_json_str(game_state_json: &str) -> GameState {
        serde_json::from_str::<GameState>(game_state_json).unwrap()
    }

    fn get_current_player_hand(&self) -> Vec<CardOption> {
        match self.players.get(self.current_turn) {
            Some(player) => player.hand.clone(),
            None => panic!("current_turn out of bounds"),
        }
    }

    fn set_current_player_hand(&mut self, new_hand: Vec<CardOption>) {
        let mut player = match self.players.get(self.current_turn) {
            Some(player) => player.clone(),
            None => panic!("{:?}", "current_turn out of bounds"),
        };
        player.set_hand(new_hand);
        self.players[self.current_turn] = player;
    }

    pub fn get_available_moves(&self) -> GameMoves {
        match self.turn_phase {
            TurnPhaseEnum::WaitForTurn => GameMoves::Wait,
            TurnPhaseEnum::Draw => GameMoves::Draw(
                self.discards
                    .iter()
                    .map(|card| DrawPhaseMove::DrawFromDiscards(card.clone()))
                    .chain(std::iter::once(DrawPhaseMove::DrawFromDeck))
                    .collect::<Vec<DrawPhaseMove>>(),
            ),
            TurnPhaseEnum::Play => GameMoves::Play(Vec::new()),
        }
    }

    pub fn process_moves(&self, game_moves: GameMoves) -> GameState {
        match game_moves {
            GameMoves::Wait => self.clone(),
            GameMoves::Draw(moves) => self.process_draw_move(moves),
            GameMoves::Play(_moves) => self.clone(),
        }
    }

    fn process_draw_move(&self, moves: Vec<DrawPhaseMove>) -> GameState {
        match &moves[..] {
            [DrawPhaseMove::DrawFromDeck] => {
                // Rust can't know the contents of the deck, because that would
                // mean sending them to the frontend. So we have to trust that the
                // Flask controller is removing a card from the deck and adding it
                // to the player's hand.
                let mut new_game_state = self.clone();
                new_game_state.turn_phase = TurnPhaseEnum::Play;
                new_game_state
            }
            [DrawPhaseMove::DrawFromDiscards(card_option)] => {
                let mut discard_iterator = self.discards.iter();
                let cards_to_draw: Vec<CardOption> = discard_iterator
                    .by_ref()
                    .take_while(|x| x != &card_option)
                    .chain(std::iter::once(card_option))
                    .cloned()
                    .map(|x| CardOption::Visible(x.get_card().clone()))
                    .collect();
                let remaining_discards: Vec<CardOption> = discard_iterator.cloned().collect();
                let mut new_game_state = self.clone();
                let new_hand = self
                    .get_current_player_hand()
                    .iter()
                    .chain(cards_to_draw.iter())
                    .cloned()
                    .collect();
                new_game_state.discards = remaining_discards;
                new_game_state.turn_phase = TurnPhaseEnum::Play;
                new_game_state.set_current_player_hand(new_hand);
                new_game_state
            }
            [_, ..] => panic!("Too many draw moves submitted"),
            _ => panic!("No valid draw move submitted"),
        }
    }

    // pub fn play_move(&self)
}

#[cfg(test)]
mod gameplay_tests {
    use crate::parser::Card;

    use super::*;

    fn get_game_state() -> GameState {
        let game_state_json = r#"{"id": 1, "players": [{"id": 1, "username": "mike", "score": 0, "order_index": 1, "melds": [], "hand": [{"card_type": "visible", "card": {"suit": "diamonds", "rank": 4}}, {"card_type": "visible", "card": {"suit": "spades", "rank": 10}}, {"card_type": "visible", "card": {"suit": "spades", "rank": 9}}, {"card_type": "visible", "card": {"suit": "diamonds", "rank": 13}}, {"card_type": "visible", "card": {"suit": "spades", "rank": 3}}, {"card_type": "visible", "card": {"suit": "diamonds", "rank": 1}}, {"card_type": "visible", "card": {"suit": "diamonds", "rank": 2}}, {"card_type": "visible", "card": {"suit": "clubs", "rank": 1}}, {"card_type": "visible", "card": {"suit": "diamonds", "rank": 7}}, {"card_type": "visible", "card": {"suit": "hearts", "rank": 4}}, {"card_type": "visible", "card": {"suit": "clubs", "rank": 9}}]}, {"id": 2, "username": "kaitlin", "score": 0, "order_index": 2, "melds": [], "hand": [{"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}, {"card_type": "hidden"}]}], "current_turn": 0, "turn_phase": "DRAW", "discards": [{"card_type": "visible", "card": {"suit": "diamonds", "rank": 6}}], "name": "game one"}"#;
        GameState::new_from_json_str(game_state_json)
    }

    #[test]
    fn test_draw_from_deck() {
        let game_state = get_game_state();
        let game_moves =
            serde_json::from_str::<GameMoves>(r#"{"draw":[{"move_type":"draw_from_deck"}]}"#)
                .unwrap();
        let new_game_state = game_state.process_moves(game_moves);
        assert_eq!(new_game_state.turn_phase, TurnPhaseEnum::Play);
        assert_eq!(new_game_state.discards, game_state.discards);
        assert_eq!(new_game_state.current_turn, game_state.current_turn);
    }

    #[test]
    fn test_draw_single_card_from_discards_len_1() {
        let game_state = get_game_state();

        let game_moves =
            serde_json::from_str::<GameMoves>(r#"{"draw":[{"move_type":"draw_from_discards","card_type":"visible","card":{"rank":6,"suit":"diamonds"}}]}"#)
                .unwrap();
        let new_game_state = game_state.process_moves(game_moves);
        assert_eq!(new_game_state.turn_phase, TurnPhaseEnum::Play);
        assert_eq!(new_game_state.current_turn, game_state.current_turn);
        assert_eq!(new_game_state.discards.len(), 0);
        assert!({
            if let CardOption::Visible(discard) = &game_state.discards[0] {
                new_game_state.get_current_player_hand().iter().any(|x| {
                    if let CardOption::Visible(card) = x {
                        card == discard
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        })
    }

    #[test]
    fn test_draw_multiple_cards_from_discards() {
        let mut game_state = get_game_state();
        game_state.discards = game_state
            .discards
            .iter()
            .cloned()
            .chain(
                [
                    CardOption::Visible(Card::new(4, "diamonds".into())),
                    CardOption::Visible(Card::new(9, "clubs".into())),
                    CardOption::Visible(Card::new(8, "spades".into())),
                    CardOption::Visible(Card::new(13, "hearts".into())),
                ]
                .iter()
                .cloned(),
            )
            .collect();
        let game_moves =
            serde_json::from_str::<GameMoves>(r#"{"draw":[{"move_type":"draw_from_discards","card_type":"visible","card":{"rank":9,"suit":"clubs"}}]}"#)
                .unwrap();
        let new_game_state = game_state.process_moves(game_moves);
        assert_eq!(new_game_state.turn_phase, TurnPhaseEnum::Play);
        assert_eq!(new_game_state.current_turn, game_state.current_turn);
        assert_eq!(new_game_state.discards.len(), 2);
        assert!(game_state.discards.iter().take(3).all(|discard_option| {
            if let CardOption::Visible(discard) = discard_option {
                new_game_state.get_current_player_hand().iter().any(|x| {
                    if let CardOption::Visible(card) = x {
                        card == discard
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        }));
    }
}
