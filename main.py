from flask import Flask, render_template
from flask_mysqldb import MySQL

app = Flask(__name__)
app.debug = True

db = MySQL()

app.config['MYSQL_USER'] = 'root'
app.config['MYSQL_PASSWORD'] = 'root'
app.config['MYSQL_DB'] = 'shop'
app.config['MYSQL_HOST'] = 'db'

db.init_app(app)

@app.route("/")
def index():
    return render_template("index.html")


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=80)
