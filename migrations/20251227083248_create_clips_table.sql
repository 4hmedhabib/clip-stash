-- Add migration script here
CREATE TABLE clips (
    clip_id TEXT PRIMARY KEY,
    shortcode TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    title TEXT,
    posted DATETIME NOT NULL,
    expires DATETIME,
    password TEXT,
    hits INTEGER NOT NULL DEFAULT 0
);