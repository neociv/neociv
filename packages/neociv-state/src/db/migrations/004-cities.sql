CREATE TABLE IF NOT EXISTS
    cities (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        owner_id REFERENCES civs (id),
        owner_idx REFERENCES civs (idx),
        title TEXT NOT NULL,
        capitol TINYINT NOT NULL DEFAULT FALSE
    );