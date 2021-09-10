-- Add migration script here
BEGIN;
CREATE TABLE IF NOT EXISTS localisation (
    id BIGSERIAL PRIMARY KEY,
    localisation_type VARCHAR(20) CHECK(localisation_type IN ('Hub', 'Rack', 'Farm')),
    user_id INT NOT NULL REFERENCES user_account(id)
);
CREATE TABLE IF NOT EXISTS sensor (
    id BIGSERIAL PRIMARY KEY,
    sensor_type VARCHAR(11) CHECK(sensor_type IN ('temperature', 'hygro', 'EC')),
    created_at TIMESTAMP NOT NULL,
    model VARCHAR(60),
    brand VARCHAR(60)
);
CREATE TABLE IF NOT EXISTS temperature (
    measured_at TIMESTAMP NOT NULL,
    localisation_id BIGINT NOT NULL REFERENCES localisation(id),
    sensor_id BIGINT NOT NULL REFERENCES sensor(id),
    temperature REAL NOT NULL CHECK(
        temperature <= 50
        AND temperature >= 0
    )
);
CREATE TABLE IF NOT EXISTS hygrometry (
    measured_at TIMESTAMP NOT NULL,
    localisation_id BIGINT NOT NULL,
    sensor_id BIGINT NOT NULL REFERENCES sensor(id),
    hygrometry REAL NOT NULL CHECK(
        hygrometry <= 1
        AND hygrometry >= 0
    )
);
SELECT create_hypertable('temperature', 'measured_at');
SELECT create_hypertable('hygrometry', 'measured_at');
COMMIT;