CREATE TABLE IF NOT EXISTS categories(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS snippets(
    id SERIAL PRIMARY KEY NOT NULL,
    category_id INTEGER REFERENCES categories(id),
    description VARCHAR(255),
    code VARCHAR(255)
);