CREATE TABLE
    tree_defs (id VARCHAR PRIMARY KEY);

CREATE TABLE
    trees (
        civ_id REFERENCES civs (id),
        civ_idx REFERENCES civs (idx),
        tree_def REFERENCES tree_defs (id),
        state JSON NOT NULL,
        CONSTRAINT tree_key PRIMARY KEY (civ_id, civ_idx, tree_def)
    );