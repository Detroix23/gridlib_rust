// GRID
// Test functions


use grid::{self, Grid, GridKind, HashMap, TileFeatures, DEFAULT_UI_TILES, GRID_SIZE};

/// # Test function.
pub fn main() {
    // Vars
    let default_ui_features: HashMap<TileFeatures, &'static str> = HashMap::new();

    println!("Grid 1");
    let mut grid_cartesian1: Grid = Grid::new(GridKind::Squares, GRID_SIZE, false);
    grid_cartesian1.tiles.remove(34);
    grid_cartesian1.display_inline(&DEFAULT_UI_TILES, &default_ui_features);
}