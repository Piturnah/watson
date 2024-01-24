DROP TABLE user_problem;

CREATE TABLE user_problem (
    user_id VARCHAR,
    problem_id INT REFERENCES problems(id) ON UPDATE CASCADE ON DELETE CASCADE,
    last_solved TIMESTAMP NOT NULL DEFAULT NOW(),
    successful BOOLEAN NOT NULL,
    CONSTRAINT user_problem_pk PRIMARY KEY (user_id, problem_id)
);

ALTER TABLE problems DROP COLUMN user_id;
ALTER TABLE problems ADD COLUMN user_id VARCHAR;

ALTER TABLE solutions DROP COLUMN user_id;
ALTER TABLE solutions ADD COLUMN user_id VARCHAR;
