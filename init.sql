CREATE DATABASE IF NOT EXISTS `shop`;

USE `shop`;

CREATE TABLE `users` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `email` char(128) NOT NULL,
    `phone_number` char(128) NOT NULL,
    `password` char(128) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `products` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `name` char(128) NOT NULL,
    `desc` varchar(128) NOT NULL,
    `category_id` bigint(20) NOT NULL,
    `cost` float NOT NULL,
    `discount_cost` float NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `images` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `product_id` bigint(20) NOT NULL,
    `path` char(128) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY (`path`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

INSERT INTO
    `users` (`email`, `phone_number`, `password`)
VALUES
    (
        "qwe@qwe.qwe",
        "+79111111111",
        "6b6a0ee84f21fbe8a30aacbb20d19609695f9917ea0c24eede834c5985c0a047550fb448a1d20cbe8b07970196112e0464f571781c8b1c53db721db872b0241d"
    );

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