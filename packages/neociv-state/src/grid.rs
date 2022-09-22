use num_integer::div_floor;

use rlua::{Error as LuaError, FromLua, ToLua, Value as LuaValue};
use serde::{Deserialize, Serialize};

use crate::cell::Cell;

/// Contains Cells in a 1D Vec that are addressable in 2D space according to the xsize / ysize.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Grid {
    pub xsize: u8,
    pub ysize: u8,
    pub cells: Vec<Cell>,
}

impl<'lua> ToLua<'lua> for Grid {
    fn to_lua(self, ctx: rlua::Context<'lua>) -> rlua::Result<LuaValue<'lua>> {
        let grid_tbl = ctx.create_table()?;
        grid_tbl.set("xsize", self.xsize)?;
        grid_tbl.set("ysize", self.ysize)?;

        let cells = ctx.create_sequence_from(self.cells)?;
        grid_tbl.set("cells", cells)?;
        Ok(LuaValue::Table(grid_tbl))
    }
}

impl<'lua> FromLua<'lua> for Grid {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Grid {
                xsize: tbl.get("xsize")?,
                ysize: tbl.get("ysize")?,
                cells: tbl.get("cells")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Grid",
                message: None,
            }),
        }
    }
}

/// Generate an x,y tuple for a given index in the Grid's Cells
pub fn grid_i_to_xy(grid: &Grid, i: u16) -> (u8, u8) {
    // TODO: Bounds panic here?
    let cap: u16 = grid.xsize as u16 * grid.ysize as u16;
    return (
        (i % (grid.xsize as u16)) as u8,
        div_floor(i, div_floor(cap, grid.ysize as u16)) as u8,
    );
}

/// Generate an index for Grid's Cells Vec based on x,y coordinates.
pub fn grid_xy_to_i(grid: &Grid, x: u8, y: u8) -> u16 {
    // TODO: Bounds panic here?
    let cap: u16 = (grid.xsize * grid.ysize).into();
    return (y as u16 * div_floor(cap, grid.ysize as u16)) + x as u16;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_i_to_xy() {
        let grid = crate::grid::Grid {
            xsize: 4,
            ysize: 3,
            ..Default::default()
        };
        assert_eq!(grid.xsize, 4);
        assert_eq!(grid.ysize, 3);
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 0), (0, 0));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 1), (1, 0));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 2), (2, 0));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 3), (3, 0));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 4), (0, 1));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 5), (1, 1));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 6), (2, 1));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 7), (3, 1));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 8), (0, 2));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 9), (1, 2));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 10), (2, 2));
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 11), (3, 2));
    }

    #[test]
    fn test_xy_to_i() {
        let grid = crate::grid::Grid {
            xsize: 4,
            ysize: 3,
            ..Default::default()
        };
        assert_eq!(grid.xsize, 4);
        assert_eq!(grid.ysize, 3);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 0, 0), 0);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 1, 0), 1);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 2, 0), 2);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 3, 0), 3);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 0, 1), 4);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 1, 1), 5);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 2, 1), 6);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 3, 1), 7);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 0, 2), 8);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 1, 2), 9);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 2, 2), 10);
        assert_eq!(crate::grid::grid_xy_to_i(&grid, 3, 2), 11);
    }

    // This test is purely just to confirm that u8/u16 conversions
    // all behave correctly and indicies aren't overflowing.
    #[test]
    fn test_i_to_xy_large() {
        let grid = crate::grid::Grid { 
            xsize: 25,
            ysize: 11,
            ..Default::default()
        };
        assert_eq!(crate::grid::grid_i_to_xy(&grid, 274), (24, 10));
    }
}
