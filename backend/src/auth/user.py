from flask_login import UserMixin

from ..app import db
from ..games.game import UserGame


class User(UserMixin, db.Model):
    id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(1000), unique=True, nullable=False)
    password = db.Column(db.String(100), nullable=False)
    game_associations = db.relationship(
        UserGame,
        back_populates="user",
    )
    games = db.relationship(
        "Game",
        secondary=UserGame.__table__,
        back_populates="users",
        viewonly=True,
    )
    date_created = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
    )
    date_modified = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
        onupdate=db.func.current_timestamp(),
    )

    def as_dict(self):
        return {
            "id": self.id,
            "username": self.username,
        }
