import json

from ..app import db


class UserGame(db.Model):
    user_id = db.Column(
        db.Integer,
        db.ForeignKey("user.id"),
        primary_key=True,
        nullable=False,
    )
    game_id = db.Column(
        db.Integer,
        db.ForeignKey("game.id"),
        primary_key=True,
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
    # Users should be sorted into order by this index
    order_index = db.Column(db.Integer, nullable=False)
    score = db.Column(db.Integer, nullable=False, default=0)
    hand = db.Column(db.String(512))
    date_created = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
    )
    date_modified = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
        onupdate=db.func.current_timestamp(),
    )

    def load_hand(self):
        return json.loads(self.hand)

    def set_hand(self, hand):
        self.hand = json.dumps(hand)


class Game(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    users = db.relationship(
        "User",
        secondary=UserGame.__table__,
        back_populates="games",
        viewonly=True,
        order_by="desc(UserGame.order_index)",
    )
    user_associations = db.relationship(
        UserGame,
        back_populates="game",
        order_by="desc(UserGame.order_index)",
    )
    name = db.Column(db.String(1024))
    cards = db.Column(db.String(512))
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
        return {
            "id": self.id,
            "players": [
                {
                    **user_game.user.as_dict(),
                    "score": user_game.score,
                    "order_index": user_game.order_index,
                    "hand": user_game.load_hand()
                    if current_user and current_user.id == user_game.user_id
                    else ["xx" for card in user_game.load_hand()],
                }
                for user_game in self.user_associations
            ],
            "name": self.name,
        }

    def add_user(self, user):
        self.user_associations.append(
            UserGame(
                user=user,
                game=self,
                order_index=len(self.users),
            )
        )

    def add_users(self, users):
        for user in users:
            self.add_user(user)

    def set_cards(self, cards):
        self.cards = json.dumps(cards)

    def load_cards(self):
        return json.loads(self.cards)
