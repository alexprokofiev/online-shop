import jwt
from functools import wraps
from flask import request, abort
import hashlib


def auth_req(app):
    def _auth_req(f):
        @wraps(f)
        def __auth_req(*args, **kwargs):
            if request.cookies["token"] is None or not identity(
                app, request.cookies["token"]
            ):
                abort(400)

            return f(*args, **kwargs)

        return __auth_req

    return _auth_req


def authenticate(app, email: str, password: str) -> str | None:
    cursor = app.config["db"].connection.cursor()
    cursor.execute(
        f"""
            SELECT
                id
            FROM
                shop.users
            WHERE
                email = '{email}'
                AND password = '{ hashlib.sha512(
                    app.config["SECRET_KEY"].encode("utf-8") + password.encode("utf-8")
                ).hexdigest() }';
        """
    )

    user = cursor.fetchone()
    if user is None:
        return None

    token = jwt.encode(
        {"user_id": user[0]}, app.config["SECRET_KEY"], algorithm="HS256"
    )

    return token


def reg(app, email: str, password: str, phone_number: str) -> str | None:
    cursor = app.config["db"].connection.cursor()
    cursor.execute(
        f"""
            INSERT INTO
                shop.users
            (
                email,
                phone_number,
                password
            )
            VALUES (
                { email },
                { phone_number },
                { hashlib.sha512(
                    app.config["SECRET_KEY"].encode("utf-8") + password.encode("utf-8")
                ).hexdigest() }
            );
        """
    )

    user = cursor.fetchone()
    if user is None:
        return None

    token = jwt.encode(
        {"user_id": user[0]}, app.config["SECRET_KEY"], algorithm="HS256"
    )

    return token


def identity(app, token: str) -> bool:
    try:
        payload = jwt.decode(token, app.config["SECRET_KEY"], algorithms=["HS256"])
    except:
        return False

    user_id = payload["user_id"]

    cursor = app.config["db"].connection.cursor()
    cursor.execute(
        f"""
            SELECT
                id,
                email,
                phone_number,
                password
            FROM
                shop.users
            WHERE
                id = {user_id}
        """
    )

    user = cursor.fetchone()

    return user is not None
