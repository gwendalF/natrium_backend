-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS user_account (
    id SERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(64)
);
INSERT INTO user_account (email, password)
VALUES ('first@user.com', '123'),
    ('Second user email', '123') ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS provider_user_mapper(
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) UNIQUE NOT NULL,
    subject TEXT UNIQUE NOT NULL,
    user_id INT NOT NULL REFERENCES user_account(id)
);
INSERT INTO provider_user_mapper(name, subject, user_id)
VALUES (
        'google',
        'eyJhbGciOiJSUzI1NiIsImtpZCI6IjBmY2MwMTRmMjI5MzRlNDc0ODBkYWYxMDdhMzQwYzIyYmQyNjJiNmMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJhY2NvdW50cy5nb29nbGUuY29tIiwiYXpwIjoiMTA4ODk2Nzk2MDMzMC04MWtmaWJobzR1cnFrM291b2xxbmduc3ZhcnNmMTV0Ni5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsImF1ZCI6IjEwODg5Njc5NjAzMzAtODFrZmliaG80dXJxazNvdW9scW5nbnN2YXJzZjE1dDYuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMDExNjE0NDk3Njc0MTA4NjA0MzQiLCJlbWFpbCI6Imd3ZW5kYWxmZXJuZXRAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImF0X2hhc2giOiJPMjFvTnItWVZCSmtfUHR2SDh0MURBIiwibmFtZSI6Ikd3ZW5kYWwgRmVybmV0IiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FBVFhBSnlrR21TS1ExNUozRWFyWWIzQzg5OUtHVU9hVFpGZ2FSQkY2WGRiPXM5Ni1jIiwiZ2l2ZW5fbmFtZSI6Ikd3ZW5kYWwiLCJmYW1pbHlfbmFtZSI6IkZlcm5ldCIsImxvY2FsZSI6ImZyIiwiaWF0IjoxNjI4MTcxNDI5LCJleHAiOjE2MjgxNzUwMjksImp0aSI6IjY4YjgwZmI2OWQ4NjEwZTNmNTdmOTRjMmI2OGE3ZTI2M2E4MDkwODIifQ.TXqtxFggM8kC9TsBZZFDN0k0Su5FEye - _u9sAWFLUUbCjFFoJTtL - YdSVpAmrvYXGSQbiTMvPUaGxtWp - aBliA4d7ZIdQKy4oPFiqpqUuMHuDwJ65bCoj2HECgsvC__F5epNzPCzElcK9x8Kz6 - 5jpml2AeAHZWstxnUxR7Khxwm5Cypwsfdx4EMO - D6ITrRJv - h5ee8tLsXZa4lL2YF - YYIcxzo0cCXXFVWEOdJE9pI0sexV0ZJh5y9WwHOJHR5KtsUG1jNCm - ZM_ZbtX_9qa_K6NxIFrBdCutj8qldm3U_xHVP - CgXgwkXNwxXdK0NiwDABz6jUbwKpdT3Re60 - g',
        1
    ) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS plant (
    id SERIAL PRIMARY KEY,
    species VARCHAR(30) NOT NULL UNIQUE
);
INSERT INTO plant (species)
VALUES ('Tomato'),
    ('Salad'),
    ('Eggplant'),
    ('Strawberry') ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS localisation (
    id SERIAL PRIMARY KEY,
    localisation_type VARCHAR(20) CHECK (localisation_type IN ('Hub', 'Farm', 'Rack')),
    user_id INT NOT NULL REFERENCES user_account(id)
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
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    localisation_id INT NOT NULL REFERENCES localisation(id)
);
INSERT INTO hub (name, localisation_id)
VALUES ('First ever hub', 1),
    ('Hub2', 2),
    ('Hub user 2', 8) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS farm (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    plant_id INT NOT NULL REFERENCES plant(id),
    hub_id INT NOT NULL REFERENCES hub(id),
    localisation_id INT NOT NULL REFERENCES localisation(id)
);
INSERT INTO farm (name, plant_id, hub_id, localisation_id)
VALUES ('Strawberry farm', 4, 1, 3),
    ('Salad farm', 2, 1, 4),
    ('Tomato farm', 1, 2, 5),
    ('Farm user 2', 4, 3, 9) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS rack (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    farm_id INT NOT NULL REFERENCES farm(id),
    localisation_id INT NOT NULL REFERENCES localisation(id)
);
INSERT INTO rack (name, farm_id, localisation_id)
VALUES ('Strawberry rack', 1, 6),
    ('Tomato rack', 3, 7),
    ('Rack user 2', 4, 10) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS temperature (
    id SERIAL PRIMARY KEY,
    measured_at TIMESTAMP NOT NULL,
    temperature REAL NOT NULL,
    localisation_id INT NOT NULL REFERENCES localisation(id)
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