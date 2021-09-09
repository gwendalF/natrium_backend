-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS user_account (
    id SERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(116),
    salt VARCHAR(32),
    CHECK (
        (
            CASE
                WHEN password IS NULL THEN 0
                ELSE 1
            END
        ) + (
            CASE
                WHEN salt IS NULL THEN 0
                ELSE 1
            END
        ) != 1
    )
);
INSERT INTO user_account (email, password, salt)
VALUES ('first@user.com', '123', '123456789'),
    ('Second user email', '123', '012345678') ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS provider_user_mapper(
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) UNIQUE NOT NULL CHECK(name IN ('google', 'facebook')),
    subject TEXT UNIQUE NOT NULL,
    user_id INT NOT NULL REFERENCES user_account(id)
);
-- CREATE TABLE IF NOT EXISTS plant (
--     id SERIAL PRIMARY KEY,
--     species VARCHAR(30) NOT NULL UNIQUE
-- );
-- INSERT INTO plant (species)
-- VALUES ('Tomato'),
--     ('Salad'),
--     ('Eggplant'),
--     ('Strawberry') ON CONFLICT DO NOTHING;
-- CREATE TABLE IF NOT EXISTS localisation (
--     id SERIAL PRIMARY KEY,
--     localisation_type VARCHAR(20) CHECK (localisation_type IN ('Hub', 'Farm', 'Rack')),
--     user_id INT NOT NULL REFERENCES user_account(id)
-- );
-- INSERT INTO localisation (localisation_type, user_id)
-- VALUES ('Hub', 1),
--     ('Hub', 1),
--     ('Farm', 1),
--     ('Farm', 1),
--     ('Farm', 1),
--     ('Rack', 1),
--     ('Rack', 1),
--     ('Hub', 2),
--     ('Farm', 2),
--     ('Rack', 2) ON CONFLICT DO NOTHING;
-- CREATE TABLE IF NOT EXISTS hub (
--     id SERIAL PRIMARY KEY,
--     name VARCHAR(20) NOT NULL,
--     localisation_id INT NOT NULL REFERENCES localisation(id)
-- );
-- INSERT INTO hub (name, localisation_id)
-- VALUES ('First ever hub', 1),
--     ('Hub2', 2),
--     ('Hub user 2', 8) ON CONFLICT DO NOTHING;
-- CREATE TABLE IF NOT EXISTS farm (
--     id SERIAL PRIMARY KEY,
--     name VARCHAR(20) NOT NULL,
--     plant_id INT NOT NULL REFERENCES plant(id),
--     hub_id INT NOT NULL REFERENCES hub(id),
--     localisation_id INT NOT NULL REFERENCES localisation(id)
-- );
-- INSERT INTO farm (name, plant_id, hub_id, localisation_id)
-- VALUES ('Strawberry farm', 4, 1, 3),
--     ('Salad farm', 2, 1, 4),
--     ('Tomato farm', 1, 2, 5),
--     ('Farm user 2', 4, 3, 9) ON CONFLICT DO NOTHING;
-- CREATE TABLE IF NOT EXISTS rack (
--     id SERIAL PRIMARY KEY,
--     name VARCHAR(20) NOT NULL,
--     farm_id INT NOT NULL REFERENCES farm(id),
--     localisation_id INT NOT NULL REFERENCES localisation(id)
-- );
-- INSERT INTO rack (name, farm_id, localisation_id)
-- VALUES ('Strawberry rack', 1, 6),
--     ('Tomato rack', 3, 7),
--     ('Rack user 2', 4, 10) ON CONFLICT DO NOTHING;
-- CREATE TABLE IF NOT EXISTS temperature (
--     id SERIAL PRIMARY KEY,
--     measured_at TIMESTAMP NOT NULL,
--     temperature REAL NOT NULL,
--     localisation_id INT NOT NULL REFERENCES localisation(id)
-- );
-- INSERT INTO temperature (measured_at, temperature, localisation_id)
-- VALUES ('2021-07-01 06:00:00', 15.0, 6),
--     ('2021-07-01 06:15:00', 14.2, 6),
--     ('2021-07-01 06:30:00', 12.6, 6),
--     ('2021-07-02 07:30:00', 19.3, 6),
--     ('2021-07-01 06:00:00', 14.5, 7),
--     ('2021-07-01 06:15:00', 16.8, 7),
--     ('2021-07-01 06:30:00', 13.5, 7),
--     ('2021-07-02 07:30:00', 22.3, 7),
--     ('2021-07-01 06:00:00', 22.3, 10),
--     ('2021-07-02 06:15:00', 22.3, 10),
--     ('2021-07-02 06:30:00', 22.3, 10),
--     ('2021-07-02 07:30:00', 22.3, 10) ON CONFLICT DO NOTHING;
COMMIT;