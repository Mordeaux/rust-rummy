from flask import Flask, request, jsonify
from flask_cors import CORS

app = Flask(__name__)
CORS(app)

@app.route('/login', methods=['POST'])
def login():
    data = request.get_json()
    username = data.get('username')
    password = data.get('password')
    return jsonify({
        'success': True,
    })

@app.route('/')
def hello_world():
    return '<p>Hello, World!</p>'
