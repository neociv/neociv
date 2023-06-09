CREATE TABLE IF NOT EXISTS grid (
    xsize INT8 NOT NULL DEFAULT 0,
    ysize INT8 NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS cells (
    -- x,y cartesian coords on the grid
    x INT8 NOT NULL,
    y INT8 NOT NULL,
    -- Effectively "elevation"
    z INT8 NOT NULL DEFAULT 0,
    -- Masks represent whether or not the six hex neighbours match the cells own condition (eg. road) or whether
    -- or not a condition is true for an edge (eg. coast)
    terrain_mask INT8 NOT NULL DEFAULT 0,
    owner_mask INT8 NOT NULL DEFAULT 0,
    coast_mask INT8 NOT NULL DEFAULT 0,
    river_mask INT8 NOT NULL DEFAULT 0,
    road_mask INT8 NOT NULL DEFAULT 0,
    -- Base terrain type
    terrain TEXT NOT NULL CHECK (
        terrain IN ('ground', 'water', 'deepwater', 'mountain')
    ) NOT NULL DEFAULT 'ground',
    -- Terrain/cell features
    river TINYINT NOT NULL DEFAULT FALSE,
    road TINYINT NOT NULL DEFAULT FALSE,
    -- Owner but can be "unowned"
    owner_id VARCHAR REFERENCES civs (id),
    -- All selections will be done on the x & y coordinates
    CONSTRAINT coords PRIMARY KEY (x, y),
    -- If a civ is deleted set owner to null
    CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES civs(id) ON DELETE
    SET NULL
);