ALTER TABLE users DROP CONSTRAINT users_pkey CASCADE;
ALTER TABLE users DROP COLUMN id;
ALTER TABLE users ADD COLUMN id VARCHAR NOT NULL DEFAULT random()::text;
ALTER TABLE users ADD PRIMARY KEY (id);

ALTER TABLE users DROP COLUMN password;