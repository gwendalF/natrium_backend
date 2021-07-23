-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS user_account (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(128) UNIQUE,
    password VARCHAR(128)
);
INSERT INTO user_account (email, password)
VALUES ('hello@first.com', '123456'),
    ('user2', '123');
CREATE TABLE IF NOT EXISTS measurer (
    id BIGSERIAL PRIMARY KEY,
    measurer_type VARCHAR(10) CHECK (measurer_type IN ('Hub', 'Farm', 'Rack')),
    user_id BIGINT NOT NULL REFERENCES user_account(id)
);
INSERT INTO measurer (measurer_type, user_id)
VALUES ('Hub', 1),
    ('Farm', 2),
    ('Hub', 2),
    ('Rack', 2);
CREATE TABLE IF NOT EXISTS temperature (
    id BIGSERIAL PRIMARY KEY,
    measured_at TIMESTAMP NOT NULL,
    measurer_id BIGINT NOT NULL REFERENCES measurer(id),
    temperature REAL NOT NULL
);
INSERT INTO temperature (measured_at, measurer_id, temperature)
VALUES ('2021-06-20 06:30:20', 1, 15.3),
    ('2021-06-20 06:45:00', 1, 12.3),
    ('2021-06-20 07:00:00', 1, 25.3),
    ('2021-06-20 05:30:00', 4, 26.1),
    ('2021-10-30 12:00:00', 4, 27.6);
COMMIT;