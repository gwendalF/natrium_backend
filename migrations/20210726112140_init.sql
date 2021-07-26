-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS user_account (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(64)
);
INSERT INTO user_account (email, password)
VALUES ('first@user.com', '123'),
    ('Second user email', '123') ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS plant (
    id BIGSERIAL PRIMARY KEY,
    species VARCHAR(30) NOT NULL UNIQUE
);
INSERT INTO plant (species)
VALUES ('Tomato'),
    ('Salad'),
    ('Eggplant'),
    ('Strawberry') ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS localisation (
    id BIGSERIAL PRIMARY KEY,
    localisation_type VARCHAR(20) CHECK (localisation_type IN ('Hub', 'Farm', 'Rack')),
    user_id BIGINT NOT NULL REFERENCES user_account(id)
);
INSERT INTO localisation (localisation_type, user_id)
VALUES ('Hub', 1),
    ('Hub', 1),
    ('Farm', 1),
    ('Farm', 1),
    ('Farm', 1),
    ('Rack', 1),
    ('Rack', 1),
    ('Hub', 2),
    ('Farm', 2),
    ('Rack', 2) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS hub (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    localisation_id BIGINT NOT NULL REFERENCES localisation(id)
);
INSERT INTO hub (name, localisation_id)
VALUES ('First ever hub', 1),
    ('Hub2', 2),
    ('Hub user 2', 8) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS farm (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    plant_id BIGINT NOT NULL REFERENCES plant(id),
    hub_id BIGINT NOT NULL REFERENCES hub(id),
    localisation_id BIGINT NOT NULL REFERENCES localisation(id)
);
INSERT INTO farm (name, plant_id, hub_id, localisation_id)
VALUES ('Strawberry farm', 4, 1, 3),
    ('Salad farm', 2, 1, 4),
    ('Tomato farm', 1, 2, 5),
    ('Farm user 2', 4, 3, 9) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS rack (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    farm_id BIGINT NOT NULL REFERENCES farm(id),
    localisation_id BIGINT NOT NULL REFERENCES localisation(id)
);
INSERT INTO rack (name, farm_id, localisation_id)
VALUES ('Strawberry rack', 1, 6),
    ('Tomato rack', 3, 7),
    ('Rack user 2', 4, 10) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS temperature (
    id BIGSERIAL PRIMARY KEY,
    measured_at TIMESTAMP NOT NULL,
    temperature REAL NOT NULL,
    localisation_id BIGINT NOT NULL REFERENCES localisation(id)
);
INSERT INTO temperature (measured_at, temperature, localisation_id)
VALUES ('2021-07-01 06:00:00', 15.0, 6),
    ('2021-07-01 06:15:00', 14.2, 6),
    ('2021-07-01 06:30:00', 12.6, 6),
    ('2021-07-02 07:30:00', 19.3, 6),
    ('2021-07-01 06:00:00', 14.5, 7),
    ('2021-07-01 06:15:00', 16.8, 7),
    ('2021-07-01 06:30:00', 13.5, 7),
    ('2021-07-02 07:30:00', 22.3, 7),
    ('2021-07-01 06:00:00', 22.3, 10),
    ('2021-07-02 06:15:00', 22.3, 10),
    ('2021-07-02 06:30:00', 22.3, 10),
    ('2021-07-02 07:30:00', 22.3, 10) ON CONFLICT DO NOTHING;
COMMIT;