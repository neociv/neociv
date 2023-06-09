CREATE TABLE IF NOT EXISTS cities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    owner_id REFERENCES civs (id),
    title TEXT NOT NULL,
    capitol TINYINT NOT NULL DEFAULT FALSE,
    CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES civs(id) ON DELETE
    SET NULL
);