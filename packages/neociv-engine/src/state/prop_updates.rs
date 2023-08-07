use neociv_db::NeocivDB;

use neociv_state::{cell::Cell, grid::Grid};

pub fn update_grid(db: &NeocivDB) -> Grid {
    Grid {
        xsize: 0,
        ysize: 0,
        cells: update_grid_cells(db),
    }
}

pub fn update_grid_cells(db: &NeocivDB) -> Vec<Cell> {
    return vec![];
}
