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
        "1faf5f5e23dfa747a36f39e7c1e951ef8ec698b9e789e5e821f2ef5509c922719bf43adabd0d417e4b78939b421132fa130dd858aea19469531666d3974bdb5b"
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