// GRID
// Test functions


use grid::{self, grid_init, grid_inline, Grid, GridKind, HashMap, TileFeatures, DEFAULT_UI_TILES, GRID_SIZE};

/// # Test function.
pub fn main() {
    // Vars
    let default_ui_features: HashMap<TileFeatures, &'static str> = HashMap::new();

    println!("Grid 1");
    let mut grid_cartesian1: Grid = grid_init(GridKind::Squares, GRID_SIZE, false);
    grid_cartesian1.tiles.remove(34);
    grid_inline(&grid_cartesian1, &DEFAULT_UI_TILES, &default_ui_features);
}