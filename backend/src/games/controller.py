"""Controller for dealing with game routes."""
import json
import random

import rummy

from .game import Game, UserGame


class GameController:
    """Methods to facilitate gameplay."""

    def __init__(
        self, player: "User", game: Game = None, user_game: UserGame = None
    ):
        self.player = player
        if game:
            self.game = game
            self.user_game = next(
                (
                    user_game
                    for user_game in player.game_associations
                    if user_game.game == game
                ),
                None,
            )
        elif user_game:
            self.user_game = user_game
            self.game = user_game.game
        else:
            raise Exception(
                "Must supply User and either a Game or UserGame instance"
            )

    def deal_new_game(self):
        """Don't forget to shuffle first."""
        cards = self.game.deck
        for user_game in self.game.user_associations:
            user_game.hand = [cards.pop() for i in range(10)]
        self.game.deck = cards

    def shuffle(self):
        """Creates a new deck and shuffles it."""

        def _cards(card_list):
            return [
                {
                    "card_type": "visible",
                    "card": {
                        "playable": True,
                        **card,
                    },
                }
                for card in card_list
            ]

        suits = ["hearts", "spades", "diamonds", "clubs"]
        cards = _cards(
            [
                {"suit": suit, "rank": rank}
                for rank in range(1, 14)
                for suit in suits
            ]
        )
        random.shuffle(cards)
        starting_discard = cards.pop()
        self.game.deck = cards
        self.game.discards = [starting_discard]

    def move(self, game_move):
        """Updates the database based on the move submitted,
        if it is a valid move."""
        json_game = json.dumps(self.game.to_dict(self.player))
        available_moves = json.loads(rummy.get_available_moves(json_game))[
            "available_moves"
        ]
        new_game_state = None

        if (
            game_move in available_moves["draw"]
            or game_move in available_moves["play"]
        ):
            if game_move["move_type"] == "draw_from_deck":
                new_game_state = json.loads(self._draw_from_deck(game_move))
            else:
                game_state = json.dumps(self.game.to_dict(self.player))
                move_obj = {}
                move_obj[list(available_moves.keys())[0]] = [game_move]
                new_game_state = json.loads(
                    rummy.process_move(game_state, json.dumps(move_obj))
                )

            self.game.update_from_dict(new_game_state)
        return self.game.to_dict(self.player)

    def _draw_from_deck(self, game_move) -> str:
        """Has the player associated with user_game draw one card
        from the deck."""
        deck = self.game.deck
        card = deck.pop()
        game_move = {
            **game_move,
            **card,
        }
        game_state = json.dumps(self.game.to_dict(self.player))

        self.game.deck = deck
        return rummy.process_move(
            game_state, json.dumps({"draw": [game_move]})
        )

    @staticmethod
    def get_game_by_id(player: "User", game_id: int) -> Game:
        """"""
        return next(
            (game for game in player.games if game.id == game_id), None
        )

    @staticmethod
    def get_controller_by_game_id(player: "User", game_id: int):
        """Return a GameController instance appropriate to
        the User - Game pairing."""
        return GameController(
            player, game=GameController.get_game_by_id(player, game_id)
        )
