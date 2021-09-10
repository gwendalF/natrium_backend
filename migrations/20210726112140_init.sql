-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS user_account (
    id SERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(116),
    salt VARCHAR(32),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
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
INSERT INTO user_account (email, password, salt, created_at, updated_at)
VALUES (
        'first@user.com',
        '123',
        '123456789',
        '2021-01-01 12:00:00',
        '2021-01-01 12:00:00'
    ),
    (
        'Second user email',
        '123',
        '012345678',
        '2021-01-01 12:00:00',
        '2021-01-01 12:00:00'
    ) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS provider_user_mapper(
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) UNIQUE NOT NULL CHECK(name IN ('google', 'facebook')),
    subject TEXT UNIQUE NOT NULL,
    user_id INT NOT NULL REFERENCES user_account(id)
);
COMMIT;