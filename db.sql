CREATE TABLE IF NOT EXISTS commandtable
(
    id SERIAL PRIMARY KEY NOT NULL,
    category VARCHAR(255),
    description VARCHAR(255),
    code VARCHAR(255)
);