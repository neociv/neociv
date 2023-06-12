CREATE TABLE IF NOT EXISTS currency_defs (
    id VARCHAR PRIMARY KEY,
    title TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS currencies (
    civ_id REFERENCES civs (id),
    currency_def REFERENCES currency_defs (id),
    CONSTRAINT currency_key PRIMARY KEY (civ_id, currency_def),
    CONSTRAINT fk_civ_id FOREIGN KEY (civ_id) REFERENCES civs(id) ON DELETE CASCADE
);