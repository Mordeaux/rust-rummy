"""Controller for dealing with game routes."""
import random

from .game import Game, UserGame


class GameController:
    """Methods to facilitate gameplay."""

    def __init__(self, player: "User", game: Game = None, user_game: UserGame = None):
        self.player = player
        if game:
            self.game = game
            self.user_game = next((user_game for user_game in player.game_associations if user_game.game == game), None)
        elif user_game:
            self.user_game = user_game
            self.game = user_game.game
        else:
            raise Exception("Must supply User and either a Game or UserGame instance")

    def deal_new_game(self):
        """Don't forget to shuffle first."""
        cards = self.game.deck
        for user_game in self.game.user_associations:
            user_game.hand = [cards.pop() for i in range(10)]
        self.game.deck = cards

    def shuffle(self):
        """Creates a new deck and shuffles it."""
        suits = ["hearts", "spades", "diamonds", "clubs"]
        cards = [{'suit':suit, 'rank': rank} for rank in range(1, 14) for suit in suits]
        random.shuffle(cards)
        starting_discard = cards.pop()
        self.game.deck = cards
        self.game.discards = [starting_discard]

    def draw(self):
        """Has the player associated with user_game draw one card."""
        deck = self.game.deck
        hand = self.user_game.hand

        if len(hand) <= 10:
            hand.append(deck.pop())

            self.user_game.hand = hand
            self.game.deck = deck

    @staticmethod
    def get_game_by_id(player: "User", game_id: int) -> Game:
        """"""
        return next((game for game in player.games if game.id == game_id), None)

    @staticmethod
    def get_controller_by_game_id(player: "User", game_id: int):
        """Return a GameController instance appropriate to the User - Game pairing."""
        return GameController(player, game=GameController.get_game_by_id(player, game_id))
