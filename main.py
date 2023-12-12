from flask import Flask, render_template
from flask_mysqldb import MySQL

app = Flask(__name__)

db = MySQL()

app.config['MYSQL_USER'] = 'root'
app.config['MYSQL_PASSWORD'] = 'root'
app.config['MYSQL_DB'] = 'shop'
app.config['MYSQL_HOST'] = 'db'

app.config['TEMPLATES_AUTO_RELOAD'] = True
db.init_app(app)

@app.route("/")
def index():
    return render_template("index.html")

@app.route("/home")
def home():
    return render_template("home.html")

@app.route("/catalog")
def catalog():
    return render_template("catalog.html")

@app.route("/login")
def login():
    return render_template("login.html")

@app.route("/cart")
def cart():
    return render_template("cart.html")

@app.route("/connect")
def connect():
    return render_template("connect.html")

@app.route("/main")
def main():
    return render_template("main.html")
