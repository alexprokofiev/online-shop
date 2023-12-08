FROM python:3.11.7-slim


RUN apt update && apt install pkg-config build-essential libmariadb-dev -y

RUN pip3 install --upgrade pip && pip3 install -U Flask flask-mysqldb

WORKDIR /app

COPY . /app

EXPOSE 80

ENTRYPOINT [ "python3", "main.py" ]
