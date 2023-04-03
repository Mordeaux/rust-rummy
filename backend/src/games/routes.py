from flask import Blueprint, Response, jsonify, request
from flask_login import current_user, login_required

from ..app import db
from .controller import GameController
from .game import Game


def create_games_blueprint():
    games = Blueprint("games", __name__, url_prefix="/games")

    @games.route("/", methods=["POST"])
    @login_required
    def create_game() -> Response:
        from ..auth.user import User

        data = request.get_json()
        name: str = data.get("name", "")

        user_ids = [1, 2]
        users = [User.query.get(user_id) for user_id in user_ids]

        new_game = Game(
            name=name,
            users=users,
        )
        db.session.add(new_game)
        db.session.commit()

        return jsonify(new_game.to_dict(current_user))

    @games.route("/", methods=["GET"])
    @login_required
    def list_games() -> Response:
        return jsonify(
            [game.to_dict(current_user) for game in current_user.games]
        )

    @games.route("/<int:game_id>", methods=["PUT"])
    @login_required
    def move(game_id: int) -> Response:
        """"""
        game_move = request.get_json()
        game_controller = GameController.get_controller_by_game_id(
            current_user, game_id
        )
        new_game_state = game_controller.move(game_move)
        db.session.add(game_controller.game)
        db.session.commit()
        return (
            jsonify(new_game_state)
            if new_game_state
            else jsonify({"error": "invalid move"})
        )

    @games.route("/<int:game_id>", methods=["GET"])
    @login_required
    def get_game_by_id(game_id: int) -> Response:
        game = GameController.get_game_by_id(current_user, game_id)
        return jsonify(game.to_dict(current_user) if game else {})

    @games.route("/<int:game_id>/join", methods=["PUT"])
    @login_required
    def join_game(game_id: int) -> Response:
        game = Game.query.get(game_id)
        game.users.append(current_user)
        db.session.add(game)
        db.session.commit()
        return jsonify(game.to_dict(current_user))

    return games
