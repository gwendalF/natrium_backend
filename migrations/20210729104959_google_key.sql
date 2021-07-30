-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS token_key (
    id SERIAL PRIMARY KEY,
    kid TEXT NOT NULL,
    provider VARCHAR(30) NOT NULL,
    modulus TEXT NOT NULL,
    exponent TEXT NOT NULL,
    expiration TIMESTAMP NOT NULL
);
INSERT INTO token_key (kid, provider, modulus, exponent, expiration)
VALUES (
        '3df0a831e093fae1e24d77d47834405f95d17b54',
        'google',
        'psh4_fDTsNZ1JkC2BV6nsU7681neTu8D37bMwTzzT-hugnePDyLaR8a_2HnqJaABndr0793WQCkiDolIjX1wn0a6zTpdgCJL-vaFe2FqPg19TWsZ8O6oKZc_rtWu-mE8Po7RGzi9qPLv9FxJPbiGq_HnMUo0EG7J4sN3IuzbU--Wmuz8LWALwmfpE9CfOym8x5GdUzbDL1ltuC2zXCaxARDnPs6vKR6eW1MZgXqgQ6ZQO9FklH_b5WJYLBDmHAb6CguoeU-AozaoVrBHgkWoDkku7nMWoetULtgBP_tYtFM8zvJ9IDD6abZM0jl-bsHIm3XFz0MgAJ9FmPti9-iShQ',
        'AQAB',
        NOW() - INTERVAL '23 hour'
    ),
    (
        '0fcc014f22934e47480daf107a340c22bd262b6c',
        'google',
        '7qnlkR2Ysvik__jqELu5__2Ib4_Pix6NEmEYKY80NyIGBhUQ0QDtijFypOk3cN3aRgb1f3741vQu7PQGMr79J8jM4-sA1A6UQNmfjl-thB5JpdfQrS1n3EpsrPMUvf5w-uBMQnxmiM3hrHgjA107-UxLF_xBG8Vp_EXmZI7y6IfUwTHrNotSpLLBSNH77C8ncFcm9ADsdl-Bav2CjOaef6CpGISCscx2T4LZS6DIafU1M_xYcx3aLET9TojymjZJi2hfZDyF9x_qssrlnxqfgrI71warY8HiXsiZzOTNB6s81Fu9AaxV7YckfLHyvXwOX8lQN53c2IiAuk-T7nf69w',
        'AQAB',
        NOW() - INTERVAL '23 hour'
    );
COMMIT;