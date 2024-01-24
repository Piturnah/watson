CREATE TABLE access_tokens (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    redeemed BOOLEAN NOT NULL
);
