from flask import Flask, render_template, request, abort, make_response
from flask_mysqldb import MySQL
from auth import auth_req, authenticate, reg

app = Flask(__name__)

app.config["MYSQL_USER"] = "root"
app.config["MYSQL_PASSWORD"] = "root"
app.config["MYSQL_DB"] = "shop"
app.config["MYSQL_HOST"] = "db"

app.config["TEMPLATES_AUTO_RELOAD"] = True

app.config["SECRET_KEY"] = "polina loh"

db = MySQL(app)

app.config["db"] = db


# @app.route("/test")
# @auth_req(app)
# def test():
#     pass


@app.route("/")
def index():
    return render_template("index.html")


@app.route("/home")
def home():
    return render_template("home.html")


@app.route("/catalog")
def catalog():
    return render_template("catalog.html")


@app.route("/login", methods=["GET"])
def login_get():
    return render_template("login.html", signin=True)


@app.route("/signup", methods=["GET"])
def signup_get():
    return render_template("login.html", signin=False)


@app.route("/signup", methods=["POST"])
def signup_post():
    email = request.form.get("email")
    password = request.form.get("password")
    phone_number = request.form.get("phone_number")

    if email is None or password is None:
        abort(400)

    token = reg(app, email, password, phone_number)
    if token is None:
        abort(403)

    response = make_response(render_template("index.html"))
    response.set_cookie("token", token)

    return response

@app.route("/login", methods=["POST"])
def login_post():
    email = request.form.get("email")
    password = request.form.get("password")

    if email is None or password is None:
        abort(400)

    token = authenticate(app, email, password)
    if token is None:
        abort(403)

    response = make_response(render_template("index.html"))
    response.set_cookie("token", token)

    return response


@app.route("/cart")
def cart():
    return render_template("cart.html")


@app.route("/connect")
def connect():
    return render_template("connect.html")


@app.route("/main")
def main():
    return render_template("main.html")
