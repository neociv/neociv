CREATE TABLE IF NOT EXISTS civ_def (
    id VARCHAR NOT NULL PRIMARY KEY,
    title TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS civs (
    -- Made up of the civ_def id and the index keeping count of the civs
    id VARCHAR NOT NULL PRIMARY KEY,
    -- Definition key id
    civ_id NOT NULL REFERENCES civ_def(id),
    -- Count of the number of times this civ is present
    idx INT8 NOT NULL DEFAULT -1,
    -- Whether or not the civ can be considered active in terms of gameplay
    active TINYINT NOT NULL DEFAULT FALSE,
    -- The "ckey" is the namespaced id of a civ (eg. "org.neociv.contrib.civs.example") combined
    -- with the indexed entry of that civ the number of times it's in the game (eg. 0..255).
    -- This allows for multiple instances of the same civ to be present in a game and to be
    -- addressed either in serial (eg. select all instances of a civ) or directly (ie. by
    -- including the index counter). It is the job of the engine to structure these queries from
    -- a string such as "org.neociv.contrib.civs.examples[0]" into the appropriate SQL values.
    CONSTRAINT ckey UNIQUE (civ_id, idx),
    -- If a civ definition is removed then also remove all entries
    CONSTRAINT fk_civ_id FOREIGN KEY (civ_id) REFERENCES civ_def(id) ON DELETE CASCADE
);

-- This trigger ensures that adding a new civ entry will correctly get added with the proper indexed id
CREATE TRIGGER IF NOT EXISTS civ_entry_insert
AFTER
INSERT ON civs BEGIN
UPDATE civs
SET id = New.id || '[' || (
        SELECT COUNT(*) - 1
        FROM civs
        WHERE civ_id = New.civ_id
    ) || ']',
    idx = (
        SELECT COUNT(*) - 1
        FROM civs
        WHERE civ_id = New.civ_id
    )
WHERE ROWID = New.ROWID;
END;