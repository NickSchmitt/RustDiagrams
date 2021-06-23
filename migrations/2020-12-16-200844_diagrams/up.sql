-- Your SQL goes here
CREATE TABLE diagrams (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    photo TEXT NOT NULL,
    caption TEXT NOT NULL
    -- topic VARCHAR NOT NULL,
)