-- Your SQL goes here
CREATE TABLE exhibits (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    animals INTEGER[],
    open BOOLEAN NOT NULL DEFAULT FALSE
)
