from typing import Dict

from flask import Blueprint, Response, jsonify, request
from flask_login import current_user, login_required, login_user, logout_user
from werkzeug.security import check_password_hash, generate_password_hash

from ..app import db
from .user import User


def create_auth_blueprint() -> Blueprint:
    auth = Blueprint("auth", __name__, url_prefix="/auth")

    @auth.route("/login", methods=["POST"])
    def login() -> Response:
        data: Dict[str, str] = request.get_json()
        username: str = data.get("username", "")
        password: str = data.get("password", "")

        user = User.query.filter_by(username=username).first()
        if not user or not check_password_hash(user.password, password):
            return jsonify(
                {
                    "success": False,
                    "message": "Please check credentials and try again",
                }
            )
        login_user(user)

        return jsonify(user.to_dict())

    @auth.route("/logout", methods=["GET"])
    @login_required
    def logout() -> Response:
        logout_user()
        return jsonify(
            {
                "success": True,
                "message": "Successfully logged out",
            }
        )

    @auth.route("/signup", methods=["POST"])
    def signup() -> Response:
        data = request.get_json()
        username = data.get("username")
        password = data.get("password")

        user = User.query.filter_by(username=username).first()
        if user:
            return jsonify(
                {
                    "success": False,
                    "message": "Username already in use",
                }
            )
        new_user = User(
            username=username,
            password=generate_password_hash(password, method="sha256"),
        )
        db.session.add(new_user)
        db.session.commit()

        return jsonify(new_user.to_dict())

    @auth.route("/user")
    @login_required
    def get_user() -> Response:
        return jsonify(current_user.to_dict())

    return auth
