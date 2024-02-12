import os
import pathlib
from robyn import Robyn
from robyn.templating import JinjaTemplate

current_file_path = pathlib.Path(__file__).parent.resolve()
jinja_template = JinjaTemplate(os.path.join(current_file_path, "templates"))

render_template = jinja_template.render_template

app = Robyn(__file__)

@app.get("/")
async def index(request):
    return render_template("index.html")


@app.get("/home")
async def home(request):
    return render_template("home.html")


@app.get("/catalog")
async def catalog(request):
    return render_template("catalog.html")


@app.get("/login")
async def login_get(request):
    return render_template("login.html", signin=True)


@app.get("/signup")
async def signup_get(request):
    return render_template("login.html", signin=False)


@app.post("/signup")
async def signup_post(request):
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

@app.post("/login")
async def login_post(request):
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


@app.get("/cart")
async def cart(request):
    return render_template("cart.html")


@app.get("/connect")
async def connect(request):
    return render_template("connect.html")


app.start(host="0.0.0.0", port=80)
