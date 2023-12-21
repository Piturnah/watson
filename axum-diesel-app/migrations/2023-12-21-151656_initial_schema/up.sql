CREATE TABLE modules (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL
);

CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    module_id INT NOT NULL,
    title VARCHAR NOT NULL,
    CONSTRAINT fk_module FOREIGN KEY(module_id) REFERENCES modules(id)
);

CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    body TEXT NOT NULL,
    author VARCHAR,
    source VARCHAR
);

-- A problem can have associated many topics.
CREATE TABLE problem_topic (
    problem_id INT REFERENCES problems(id) ON UPDATE CASCADE ON DELETE CASCADE,
    topic_id INT REFERENCES topics(id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT problem_topic_pk PRIMARY KEY (problem_id, topic_id)
);

CREATE TABLE solutions (
    id SERIAL PRIMARY KEY,
    problem_id INT NOT NULL,
    body TEXT NOT NULL,
    author VARCHAR,
    CONSTRAINT fk_problem FOREIGN KEY(problem_id) REFERENCES problems(id)
);
