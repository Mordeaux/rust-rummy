"""Contains the Game model as well as UserGame pivot model."""
import json

from ..app import db


class UserGame(db.Model):
    """This table connects a user to a game. Because there is information
    relevant to this relationship, like a score, or the order index,
    a Model is used instead of the more typical Table object."""
    user_id = db.Column(
        db.Integer,
        db.ForeignKey("user.id"),
        nullable=False,
    )
    game_id = db.Column(
        db.Integer,
        db.ForeignKey("game.id"),
        nullable=False,
    )
    user = db.relationship(
        "User",
        back_populates="game_associations",
    )
    game = db.relationship(
        "Game",
        back_populates="user_associations",
    )
    # Users should be sorted into playing order by this index
    order_index = db.Column(db.Integer, primary_key=True)
    score = db.Column(db.Integer, nullable=False, default=0)
    _hand = db.Column(db.String(512))
    date_created = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
    )
    date_modified = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
        onupdate=db.func.current_timestamp(),
    )

    @property
    def hand(self):
        return json.loads(self._hand) if self._hand else []

    @hand.setter
    def hand(self, new_hand):
        self._hand = json.dumps(new_hand)


class Game(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    users = db.relationship(
        "User",
        secondary=UserGame.__table__,
        back_populates="games",
        viewonly=True,
        order_by="asc(UserGame.order_index)",
    )
    user_associations = db.relationship(
        UserGame,
        back_populates="game",
        order_by="asc(UserGame.order_index)",
    )
    name = db.Column(db.String(1024))
    _deck = db.Column(db.String(512))
    _discards = db.Column(db.String(512))
    turn = db.Column(db.Integer, default=0)
    date_created = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
    )
    date_modified = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
        onupdate=db.func.current_timestamp(),
    )

    def as_dict(self, current_user=None):
        """For JSON convenience"""
        return {
            "id": self.id,
            "players": [
                {
                    **user_game.user.as_dict(),
                    "score": user_game.score,
                    "order_index": user_game.order_index,
                    "hand": user_game.hand
                    if current_user and current_user.id == user_game.user_id
                    else ["xx" for card in user_game.hand],
                }
                for user_game in self.user_associations
            ],
            "turn": self.turn,
            "discards": self.discards,
            "name": self.name,
        }

    def add_user(self, user):
        """Add a single User object to the game."""
        self.user_associations.append(
            UserGame(
                user=user,
                game=self,
            )
        )

    def add_users(self, users):
        """Add several User objects to the game."""
        for user in users:
            self.add_user(user)

    @property
    def deck(self):
        return json.loads(self._deck) if self._deck else []

    @deck.setter
    def deck(self, new_cards):
        self._deck = json.dumps(new_cards)

    @property
    def discards(self):
        return json.loads(self._discards) if self._discards else []

    @discards.setter
    def discards(self, new_cards):
        self._discards = json.dumps(new_cards)
