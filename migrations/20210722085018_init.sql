-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS user (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(128) UNIQUE,
    password VARCHAR(128)
);
INSERT INTO user (email, password)
VALUES ('hello@first.com', '123456');
CREATE TABLE IF NOT EXISTS sensor (
    id BIGSERIAL PRIMARY KEY,
    brand VARCHAR(50),
    minimum_value real,
    maximum_value real,
    reference VARCHAR(60)
);
CREATE TABLE IF NOT EXISTS measure (
    id BIGSERIAL PRIMARY KEY,
    value REAL NOT NULL,
    measurer_id BIGINT,
    data_type
);
COMMIT;