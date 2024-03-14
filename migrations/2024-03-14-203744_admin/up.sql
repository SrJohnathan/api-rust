-- Your SQL goes here
CREATE TABLE admin (
                       id SERIAL PRIMARY KEY,
                       name varchar NOT NULL,
                       email varchar NOT NULL,
                        password varchar not null
)