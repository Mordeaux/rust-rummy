from flask import Flask
from flask_cors import CORS
from flask_sqlalchemy import SQLAlchemy
from flask_login import LoginManager


db = SQLAlchemy()

# poetry run flask --app=src --debug run

def create_app():
    from .auth import auth
    app = Flask(__name__)
    app.config['SECRET_KEY'] = 'temporary key'
    app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///db.sqlite'
    app.config['CORS_SUPPORTS_CREDENTIALS'] = True

    # Register Blueprints
    app.register_blueprint(auth)

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
