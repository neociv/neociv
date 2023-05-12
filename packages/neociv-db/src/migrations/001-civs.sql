CREATE TABLE IF NOT EXISTS
    civs (
        id VARCHAR NOT NULL,
        idx INT8 NOT NULL,
        -- Locale appropriate title of the civ
        title TEXT NOT NULL,
        -- Whether or not the civ can be considered active in terms of gameplay
        active TINYINT NOT NULL DEFAULT FALSE,
        -- The "ckey" is the namespaced id of a civ (eg. "org.neociv.contrib.civs.example") combined
        -- with the indexed entry of that civ the number of times it's in the game (eg. 0..255).
        -- This allows for multiple instances of the same civ to be present in a game and to be
        -- addressed either in serial (eg. select all instances of a civ) or directly (ie. by
        -- including the index counter). It is the job of the engine to structure these queries from
        -- a string such as "org.neociv.contrib.civs.examples[0]" into the appropriate SQL values.
        CONSTRAINT ckey PRIMARY KEY (id, idx)
    );