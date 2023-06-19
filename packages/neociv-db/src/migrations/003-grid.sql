CREATE TABLE IF NOT EXISTS cells (
    -- Order of insertion - this corresponds to the vector of cells that the engine
    -- uses to represent the state.
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- x,y cartesian coords on the grid
    x INT8 NOT NULL,
    y INT8 NOT NULL,
    -- Effectively "elevation"
    z INT8 NOT NULL DEFAULT 0,
    -- Masks represent whether or not the six hex neighbours match the cells own
    -- condition (eg. road) or whether or not a condition is true for an edge (eg.
    -- coast)
    terrain_mask INT8 NOT NULL DEFAULT 0,
    owner_mask INT8 NOT NULL DEFAULT 0,
    coast_mask INT8 NOT NULL DEFAULT 0,
    river_mask INT8 NOT NULL DEFAULT 0,
    road_mask INT8 NOT NULL DEFAULT 0,
    wall_mask INT8 NOT NULL DEFAULT 0,
    -- Base terrain type
    terrain TEXT NOT NULL CHECK (
        terrain IN ('ground', 'water', 'deepwater', 'mountain')
    ) NOT NULL DEFAULT 'ground',
    -- Terrain/cell features
    river TINYINT NOT NULL DEFAULT FALSE,
    road TINYINT NOT NULL DEFAULT FALSE,
    -- Owner but can be "unowned"
    owner_id VARCHAR REFERENCES civs (id),
    -- Most selections will be done on the x,y coords and therefore they must be unique
    CONSTRAINT coords UNIQUE (x, y),
    -- If a civ is deleted set owner to null
    CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES civs (id) ON DELETE
    SET NULL
);

-- Whenever the grid values in the meta table are changed then the *entire* grid
-- of cells is reset. All content such as units, improvements, etc... that are tied to
-- cells should automatically remove themselves upon deletion of their target cell and
-- thus don't need to be manually targetted here.
CREATE TRIGGER IF NOT EXISTS fill_cells
AFTER
UPDATE ON meta WHEN New.prop IS "grid.xsize" OR New.prop IS "grid.ysize"
BEGIN

-- Clear out the cells
    DELETE FROM cells;

-- Reset the sequence of IDs for the cells
    UPDATE sqlite_sequence SET seq = 0 WHERE name = "cells";

-- Create two sets of 0 - 255 to iterate over, first iterating over the x coord from 0
-- to grid.xsize and then incrementing the y coord before repeating the process.
-- In other words, it increments via the horizontal then vertical axis in 2D.
-- These values are limited by the value set in the meta table.
    INSERT INTO cells (x, y) WITH RECURSIVE 
    xaxis (x) AS (VALUES (0) UNION ALL SELECT x + 1 FROM xaxis WHERE x < 255 LIMIT 256),
    yaxis (y) AS (VALUES (0) UNION ALL SELECT y + 1 FROM yaxis WHERE y < 255 LIMIT 256)
    SELECT x,y FROM yaxis JOIN 
    xaxis WHERE xaxis.x < (SELECT value FROM meta WHERE prop = "grid.xsize")
    AND
    yaxis.y < (SELECT value FROM meta WHERE prop = "grid.ysize");

end
;
