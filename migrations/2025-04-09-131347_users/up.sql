-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    discord_id BIGINT NOT NULL,
    anilist_username VARCHAR(50) NOT NULL
);