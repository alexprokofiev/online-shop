CREATE DATABASE IF NOT EXISTS `shop`;

USE `shop`;

CREATE TABLE `users` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `email` varchar(128) NOT NULL,
    `phone_number` varchar(128) NOT NULL,
    `password` varchar(128) NOT NULL COMMENT 'в сыром виде храним пароли...',
    PRIMARY KEY (`id`),
    UNIQUE KEY (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `products` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `name` varchar(128) NOT NULL,
    `desc` varchar(128) NOT NULL,
    `category_id` bigint(20) NOT NULL,
    `cost` float NOT NULL,
    `discount_cost` float NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `images` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `product_id` bigint(20) NOT NULL,
    `path` varchar(128) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY (`path`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

INSERT INTO
    `users` (`email`, `phone_number`, `password`)
VALUES
    ("qwe@qwe.qwe", "+79111111111", "qwe");

INSERT INTO
    `products` (
        `name`,
        `desc`,
        `category_id`,
        `cost`,
        `discount_cost`
    )
VALUES
    ("Продукт 1", "Описание продукта 1", 1, 1000, 500);

INSERT INTO
    `images` (`product_id`, `path`)
VALUES
    (1, "/static/img/test.jpg"),
    (1, "/static/img/test1.jpg");