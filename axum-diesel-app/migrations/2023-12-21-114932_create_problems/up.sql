CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    body TEXT NOT NULL,
    author VARCHAR,
    source VARCHAR
)
