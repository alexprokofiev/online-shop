CREATE DATABASE IF NOT EXISTS `shop` CHARACTER SET utf8 COLLATE utf8_general_ci;

SET NAMES utf8;

USE `shop`;

CREATE TABLE `users` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `email` char(128) NOT NULL,
    `phone_number` char(128) NOT NULL DEFAULT "",
    `password` char(128) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `products` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `name` char(128) NOT NULL DEFAULT "",
    `category_id` bigint(20) NOT NULL DEFAULT 0,
    `cost` float NOT NULL DEFAULT 0,
    `desc` char(128) NOT NULL DEFAULT "",
    `image` char(128) NOT NULL DEFAULT "",
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `orders` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `user_id` bigint(20) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `order_products` (
    `order_id` bigint(20) NOT NULL DEFAULT 0,
    `product_id` bigint(20) NOT NULL DEFAULT 0,
    `quantity` int NOT NULL DEFAULT 0,
    PRIMARY KEY (`order_id`, `product_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

-- mb name should be primary key?
CREATE TABLE `categories` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `name` char(128) NOT NULL,
    PRIMARY KEY (`id`)
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
        `category_id`,
        `cost`,
        `image`
    )
VALUES
    ("Худи", 1, 1999, "/static/img/hoodie.jpg"),
    ("Не худи", 1, 999, "/static/img/cl1_jacket.jpg");
