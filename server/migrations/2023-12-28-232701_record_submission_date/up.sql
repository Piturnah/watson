ALTER TABLE problems 
ADD column submitted_at 
TIMESTAMP NOT NULL DEFAULT NOW();

ALTER TABLE solutions 
ADD column submitted_at 
TIMESTAMP NOT NULL DEFAULT NOW();
