-- Your SQL goes here
CREATE TABLE links (
    id INTEGER NOT NULL PRIMARY KEY,
    url_id VARCHAR NOT NULL UNIQUE,
    long_url VARCHAR NOT NULL
)
