import random

from .game import Game


class GameController:
    """"""

    def __init__(self, game: Game):
        self.game = game

    def deal_new_game(self):
        """Don't forget to shuffle first."""
        cards = self.game.load_cards()
        for user_game in self.game.user_associations:
            user_game.set_hand([cards.pop() for i in range(10)])
        self.game.set_cards(cards)

    def shuffle(self):
        suits = ["h", "s", "d", "c"]
        cards = [str(value) + suit for value in range(1, 14) for suit in suits]
        random.shuffle(cards)
        self.game.set_cards(cards)
