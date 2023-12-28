CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);

CREATE TABLE user_problem (
    user_id VARCHAR REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
    problem_id INT REFERENCES problems(id) ON UPDATE CASCADE ON DELETE CASCADE,
    last_solved TIMESTAMP NOT NULL DEFAULT NOW(),
    successful BOOLEAN NOT NULL,
    CONSTRAINT user_problem_pk PRIMARY KEY (user_id, problem_id)
);
