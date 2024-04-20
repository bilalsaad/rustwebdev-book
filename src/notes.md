


# Postgres

setting up postgres sucks

I used docker but couldn't use port 5432 idk why.


```

CREATE DATABASE rustwebdev;

CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    question_id integer REFERENCES questions,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

```
