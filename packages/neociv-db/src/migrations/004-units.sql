CREATE TABLE IF NOT EXISTS unit_def (
    id VARCHAR PRIMARY KEY,
    title TEXT NOT NULL,
    melee_enabled TINYINT NOT NULL DEFAULT FALSE,
    melee_attack INT8 NOT NULL DEFAULT 0,
    melee_defense INT8 NOT NULL DEFAULT 0,
    ranged_enabled TINYINT NOT NULL DEFAULT FALSE,
    ranged_distance INT8 NOT NULL DEFAULT 0,
    ranged_attack INT8 NOT NULL DEFAULT 0,
    ranged_defense INT8 NOT NULL DEFAULT 0 -- TODO: Promotions array but optimised for sqlite IDK?
);
CREATE TABLE IF NOT EXISTS units (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    def REFERENCES unit_def (id),
    owner_id REFERENCES civs (id),
    stats JSON NOT NULL,
    CONSTRAINT fk_def FOREIGN KEY (def) REFERENCES unit_def(id) ON DELETE CASCADE,
    CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES civs(id) ON DELETE
    SET NULL
);