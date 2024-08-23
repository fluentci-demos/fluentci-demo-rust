-- Add migration script here
CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);