CREATE DATABASE IF NOT EXISTS `shop`;

USE `shop`;

CREATE TABLE `users` (
    id bigint(20) NOT NULL AUTO_INCREMENT,
    email varchar(128) NOT NULL,
    phone_number varchar(128) NOT NULL,
    password varchar(128) NOT NULL COMMENT 'в сыром виде храним пароли...',
    PRIMARY KEY (`id`),
    UNIQUE KEY (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

INSERT INTO
    `users` (email, phone_number, password)
VALUES
    ("qwe@qwe.qwe", "+79111111111", "qwe");