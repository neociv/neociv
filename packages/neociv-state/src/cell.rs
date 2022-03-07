use num_integer::div_floor;

use crate::civ::CivKey;

#[derive(Clone, Debug)]
pub enum Terrain {
    DeepWater,
    Water,
    Ground,
}

/// Representation of a single Cell in the Grid.
#[derive(Clone, Debug)]
pub struct Cell {
    /// Horizontal (East) position of the cell.
    pub x: u8,
    /// Vertical (North) position of the cell.
    pub y: u8,
    /// Civ that owns this cell, optional.
    pub owner: Option<CivKey>,
    /// Terrain
    pub terrain: Option<Terrain>,
}

/// Contains Cells in a 1D Vec that are addressable in 2D space according to the xsize / ysize.
#[derive(Clone, Default, Debug)]
pub struct Grid {
    pub xsize: u8,
    pub ysize: u8,
    pub cells: Vec<Cell>,
}

/// Generate an x,y tuple for a given index in the Grid's Cells
pub fn grid_i_to_xy(grid: &Grid, i: u16) -> (u8, u8) {
    // TODO: Bounds panic here?
    let cap: u16 = (grid.xsize * grid.ysize).into();
    return (
        i as u8 % grid.xsize,
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
        let grid = crate::cell::Grid {
            xsize: 4,
            ysize: 3,
            ..Default::default()
        };
        assert_eq!(grid.xsize, 4);
        assert_eq!(grid.ysize, 3);
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 0), (0, 0));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 1), (1, 0));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 2), (2, 0));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 3), (3, 0));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 4), (0, 1));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 5), (1, 1));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 6), (2, 1));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 7), (3, 1));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 8), (0, 2));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 9), (1, 2));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 10), (2, 2));
        assert_eq!(crate::cell::grid_i_to_xy(&grid, 11), (3, 2));
    }

    #[test]
    fn test_xy_to_i() {
        let grid = crate::cell::Grid {
            xsize: 4,
            ysize: 3,
            ..Default::default()
        };
        assert_eq!(grid.xsize, 4);
        assert_eq!(grid.ysize, 3);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 0, 0), 0);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 1, 0), 1);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 2, 0), 2);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 3, 0), 3);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 0, 1), 4);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 1, 1), 5);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 2, 1), 6);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 3, 1), 7);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 0, 2), 8);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 1, 2), 9);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 2, 2), 10);
        assert_eq!(crate::cell::grid_xy_to_i(&grid, 3, 2), 11);
    }
}
