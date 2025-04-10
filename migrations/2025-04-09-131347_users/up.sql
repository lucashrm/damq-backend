-- Your SQL goes here

CREATE TABLE users (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    discord_id INTEGER NOT NULL,
    anilist_username VARCHAR(50) NOT NULL
);