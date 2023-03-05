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
    date_created = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
    )
    date_modified = db.Column(
        db.DateTime,
        default=db.func.current_timestamp(),
        onupdate=db.func.current_timestamp(),
    )


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
    )
    name = db.Column(db.String(1000))
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
            "users": [user.as_dict() for user in self.users],
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
