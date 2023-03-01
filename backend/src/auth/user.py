from flask import jsonify
from flask_login import UserMixin

from ..app import db


class User(UserMixin, db.Model):
    id = db.Column(db.Integer, primary_key=True) # primary keys are required by SQLAlchemy
    password = db.Column(db.String(100))
    username = db.Column(db.String(1000))

    def jsonify(self):
        return jsonify({
            'userId': self.id,
            'username': self.username,
            'games': [
                'game1',
                'game2',
                'game3',
            ],
        })
