-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS token_key (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,
    expiration TIMESTAMP NOT NULL,
    modulus BYTEA,
    exponent BYTEA
);
INSERT INTO token_key (name, expiration, modulus, exponent)
VALUES (
        'google',
        NOW() + INTERVAL '22850 second',
        decode(
            'psh4_fDTsNZ1JkC2BV6nsU7681neTu8D37bMwTzzT - hugnePDyLaR8a_2HnqJaABndr0793WQCkiDolIjX1wn0a6zTpdgCJL - vaFe2FqPg19TWsZ8O6oKZc_rtWu - mE8Po7RGzi9qPLv9FxJPbiGq_HnMUo0EG7J4sN3IuzbU --Wmuz8LWALwmfpE9CfOym8x5GdUzbDL1ltuC2zXCaxARDnPs6vKR6eW1MZgXqgQ6ZQO9FklH_b5WJYLBDmHAb6CguoeU-AozaoVrBHgkWoDkku7nMWoetULtgBP_tYtFM8zvJ9IDD6abZM0jl-bsHIm3XFz0MgAJ9FmPti9-iShQ',
            'base64'
        ),
        decode('AQAB', 'base64')
    ) ON CONFLICT DO NOTHING;
COMMIT;