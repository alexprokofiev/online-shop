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
        "b5ba77af1f7bda735894e746a199acb1d2c836424da2fc46bebb55423dccbff871877a30fab77a31e47b0a29ea0154882e532e9a29b220a8f2958773313bbb2a"
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