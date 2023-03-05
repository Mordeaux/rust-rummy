from flask import Blueprint, request, jsonify, Response
from flask_login import current_user, login_required

from ..app import db
from .game import Game


def create_games_blueprint():
    games = Blueprint('games', __name__, url_prefix='/games')

    @login_required
    @games.route('/', methods=['POST'])
    def create_game() -> Response:
        from ..auth.user import User

        data = request.get_json()
        name: str = data.get('name', '')

        user_ids = [ 1, 2 ]
        users = [ User.query.get(user_id) for user_id in user_ids ]

        new_game = Game(
            name=name,
            users=users,
        )
        db.session.add(new_game)
        db.session.commit()

        return jsonify(new_game.as_dict())

    @login_required
    @games.route('/', methods=['GET'])
    def list_games() -> Response:
        return jsonify(
            [ game.as_dict() for game in current_user.games ]
        )

    @login_required
    @games.route('/<int:game_id>', methods=['GET'])
    def get_game_by_id(game_id: int) -> Response:
        games = current_user.games
        game = games.query.get(game_id)
        return jsonify(game.as_dict())

    @login_required
    @games.route('/<int:game_id>', methods=['PUT'])
    def join_game(game_id: int) -> Response:
        game = Game.query.get(game_id)
        game.users.append(current_user)
        db.session.add(game)
        db.session.commit()
        return jsonify(game.as_dict())

    return games
