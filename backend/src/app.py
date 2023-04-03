import os

from flask import Flask
from flask_cors import CORS
from flask_login import LoginManager
from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()


def create_app() -> Flask:
    from .auth import auth_blueprint
    from .games import games_blueprint

    app = Flask(__name__)
    app.config["SECRET_KEY"] = "temporary key"
    db_uri = "postgresql+psycopg2://{}:{}@{}/{}".format(
        os.environ.get("PG_USER"),
        os.environ.get("PG_PASSWORD"),
        os.environ.get("PG_HOST"),
        os.environ.get("PG_DBNAME"),
    )
    app.config["SQLALCHEMY_DATABASE_URI"] = db_uri
    app.config["CORS_SUPPORTS_CREDENTIALS"] = True

    # Register Blueprints
    app.register_blueprint(auth_blueprint)
    app.register_blueprint(games_blueprint)

    # Enable CORS (for now)
    CORS(app)

    # Initialize database
    db.init_app(app)

    # Configure flask-login
    from .auth.user import User

    login_manager = LoginManager()
    login_manager.init_app(app)

    @login_manager.user_loader
    def load_user(user_id):
        return User.query.get(int(user_id))

    return app
