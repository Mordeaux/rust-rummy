from flask import Blueprint, request, jsonify
from werkzeug.security import generate_password_hash, check_password_hash
from flask_login import login_user, current_user

from ..app import db
from .user import User

auth = Blueprint('auth', __name__, url_prefix='/auth')

@auth.route('/login', methods=['POST'])
def login():
    data = request.get_json()
    username = data.get('username')
    password = data.get('password')

    user = User.query.filter_by(username=username).first()
    if not user or not check_password_hash(user.password, password):
        return jsonify({
            'success': False,
            'message': 'Please check credentials and try again',
        })
    login_user(user)

    return user.jsonify()

@auth.route('/signup', methods=['POST'])
def signup():
    data = request.get_json()
    username = data.get('username')
    password = data.get('password')

    user = User.query.filter_by(username=username).first()
    if user:
        return jsonify({
            'success': False,
            'message': 'Username already in use',
        })
    new_user = User(
        username=username,
        password=generate_password_hash(password, method='sha256')
    )
    db.session.add(new_user)
    db.session.commit()

    return new_user.jsonify()

@auth.route('/user')
def get_user():
    user = User.query.get(current_user.get_id())
    print(user.username)
    return user.jsonify()
