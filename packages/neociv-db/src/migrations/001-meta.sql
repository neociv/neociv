CREATE TABLE IF NOT EXISTS meta (prop VARCHAR PRIMARY KEY, value ANY);

-- The grid defaults to 0 during migration, *any* change to these values will
-- effectively reset the game as all cells will be deleted and then recreated to match
-- the new size.
INSERT INTO meta (prop, value) VALUES ("grid.xsize", 0);
INSERT INTO meta (prop, value) VALUES ("grid.ysize", 0);
