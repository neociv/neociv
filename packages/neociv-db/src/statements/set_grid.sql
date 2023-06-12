UPDATE meta
SET value = CASE
        WHEN prop = 'grid.xsize' THEN ?1
        WHEN prop = 'grid.ysize' THEN ?2
    END;