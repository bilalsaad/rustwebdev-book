-- Add up migration script here
CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    question_id integer REFERENCES questions,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

