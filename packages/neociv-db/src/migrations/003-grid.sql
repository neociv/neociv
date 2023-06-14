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
    -- All selections will be done on the x & y coordinates
    CONSTRAINT coords PRIMARY KEY (x, y),
    -- If a civ is deleted set owner to null
    CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES civs(id) ON DELETE
    SET NULL
);

CREATE TRIGGER IF NOT EXISTS fill_cells
AFTER
UPDATE ON meta
	WHEN New.prop IS "grid.xsize" OR New.prop IS "grid.ysize" BEGIN
		
	-- Clear out the cells
	DELETE FROM cells;

	-- Recursively generate the grid coordinates
	INSERT INTO cells (x,y) WITH RECURSIVE
		xaxis(x) AS (VALUES(0) UNION ALL SELECT x+1 FROM xaxis WHERE x<255 LIMIT 256),
		yaxis(y) AS (VALUES(0) UNION ALL SELECT y+1 FROM yaxis WHERE y<255 LIMIT 256)
	SELECT x,y FROM yaxis JOIN xaxis WHERE xaxis.x < (SELECT value FROM meta WHERE prop = "grid.xsize") AND yaxis.y < (SELECT value FROM meta WHERE prop = "grid.ysize");

END;
