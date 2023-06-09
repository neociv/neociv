CREATE TABLE IF NOT EXISTS tree_defs (id VARCHAR PRIMARY KEY);
CREATE TABLE IF NOT EXISTS trees (
    civ_id REFERENCES civs (id),
    tree_def REFERENCES tree_defs (id),
    state JSON NOT NULL,
    CONSTRAINT tree_key PRIMARY KEY (civ_id, tree_def),
    CONSTRAINT fk_civ_id FOREIGN KEY (civ_id) REFERENCES civs(id) ON DELETE CASCADE
);