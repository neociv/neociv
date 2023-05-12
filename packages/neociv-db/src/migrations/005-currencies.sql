CREATE TABLE IF NOT EXISTS currency_defs (
    id VARCHAR PRIMARY KEY,
    title TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS currencies (
    civ_id REFERENCES civs (id),
    civ_idx REFERENCES civs (idx),
    currency_def REFERENCES currency_defs (id),
    CONSTRAINT currency_key PRIMARY KEY (civ_id, civ_idx, currency_def)
);