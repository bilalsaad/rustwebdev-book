


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

getting it to work was a pain -- the path to the DB needed to be without the /<db_name>
and Im not sure if I needed to do somethign with the schema acls.


# chapter 8


The API key is expected to be in an env variable called BAD_WORDS_API_KEY.
there's one in the secrets directory which isn't checked in.

