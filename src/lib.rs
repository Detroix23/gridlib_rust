// GRID
// Main file for the Detoix's grid module.
// It allows multiple structured grid of various type.

pub use std::collections::HashMap;

/// Define a size of a rectangle area.
#[derive(Debug, Clone)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

/// Define the types of tilings
#[derive(Debug, Clone)]
pub enum GridKind {
    Triangle {
        // Wip
    },
    Squares,
    Hex,
    Rectange {
        size: Size,
    }
}
/// Define a grid
#[derive(Debug, Clone)]
pub struct Grid {
    pub kind: GridKind,
    pub tiles: Vec<Tile>,
    pub size: Size,

}


/// Define a tile.
#[derive(Debug, PartialEq, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub state: bool,
    pub features: Vec<TileFeatures>,
}

/// Exhausitve list of all possible state of a tile.
#[derive(PartialEq)]
pub enum TileState {
    On,
    Off,
    Void,
}

/// Define what can be placed as a feature on tile.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum TileFeatures {
    Named(&'static str),
}

// Constants
/// Fixed size means, if false, that going on a non-existant tile will generate a new; else, it will cancel the move.
/// Grid visualization will never generate new tiles.
pub const FIXED_SIZE: bool = true;
/// Grid size. All sides are equal
pub const GRID_SIZE: usize = 20;
/// UI - Grid visualisation. Void is printed when the tile is not loaded, non-existent or the feature is unknown.
pub struct UiTiles {
    pub on: &'static str,
    pub off: &'static str,
    pub void: &'static str,
}
/// UI - Grid to print chars
pub const DEFAULT_UI_TILES: UiTiles = UiTiles {
    on: "██",
    off: "░░",
    void: "▗▘"
};

/// # Create a grid
/// Need a size for creating a square grid and default fill state.
pub fn grid_init(kind: GridKind, size: usize, default_state: bool) -> Grid {
    let mut tiles: Vec<Tile> = Vec::new();
    for x in 0..size {
        for y in 0..size {
            tiles.push(Tile {
                x: x as i32,
                y: y as i32,
                state: default_state,
                features: vec![]
            });
        }
    }
    let grid: Grid = Grid {
        kind,
        tiles,
        size: Size {
            x: size,
            y: size,
        },
    };

    grid
}

/// # Use to push tiles into a grid. 
/// This function prevents to have multiple tiles with the same coord - the first will cancel any other.
pub fn add_tiles(tiles_in: Vec<Tile>) -> Vec<Tile> {
    let mut tiles_out: Vec<Tile> = Vec::new();
    for tile_in in tiles_in {
        let mut unique: bool = true;
        for tile_out in &tiles_out {
            unique = !(tile_out.x == tile_in.x && tile_out.y == tile_in.y);
        }
        if unique {
            tiles_out.push(tile_in);
        }
    }
    
    tiles_out
}

/// # Get state of a tile of a grid
/// 
pub fn grid_state_tile(grid: &Grid, x: i32, y:i32) -> TileState {
    for tile in &grid.tiles {
        if tile.x == x && tile.y == y {
            if tile.state == true {
                return TileState::On
            } else {
                return TileState::Off
            }
        }
    }

    TileState::Void
}

// # Update a tile of the grid
pub fn grid_update_tile(grid: Grid, x: i32, y:i32, state: bool, features_new: Vec<TileFeatures>) -> Grid {
    let mut grid_updated: Grid = grid.clone();
    for tile in &grid.tiles {
        if tile.x == x && tile.y == y {
            let tile_index: usize = grid.tiles.iter().position(|t| t == tile).expect("(X) - Tile somehow not found.");
            grid_updated.tiles[tile_index].state = state;
            for feature_new in features_new.clone() {
                grid_updated.tiles[tile_index].features.push(feature_new);
            }
        }
    }

    grid_updated
}

/// # Debug grid visualization
/// Allow to print inline the grid in complement with more info.
/// Use "DEFAULT_UI_TILES" to satisfy local_ui_tiles.
pub fn grid_inline(grid: &Grid, local_ui_tiles: &UiTiles, local_ui_features: &HashMap<TileFeatures, &'static str>) {
    let grid_size: usize = GRID_SIZE;
    // Find maximums
    let mut max_x: i32 = 0;
    let mut min_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut min_y: i32 = 0;
    for tile in &grid.tiles {
        if tile.x > max_x {max_x = tile.x;}
        else if tile.x < min_x {min_x = tile.x;}
        if tile.y > max_y {max_y = tile.y;}
        else if tile.y < min_y {min_y = tile.y;}
    }
    let size_x: i32 = min_x.abs() + max_x.abs();
    let size_y: i32 = min_y.abs() + max_y.abs();
    println!("- Max: x={}, y={}; Min: x={}, y={};", max_x, max_y, min_x, min_y);
    println!("- Size: x={}, y={}; Center: x={}, y={}", size_x, size_y, size_x / 2, size_y / 2);
    println!("- Legend: on={}, off={}, void={}", local_ui_tiles.on, local_ui_tiles.off, local_ui_tiles.void);
    // Iterate through the grid
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            //print!("({}; {})", x, y);
            // Check the tiles
            let mut found: bool = false;
            let mut i: usize = 0;
            while !found && i < grid.tiles.len() {
                let tile = &grid.tiles[i];
                // If tile exist and is found.
                if tile.x == x as i32 && tile.y == y as i32 {
                    if tile.features.len() > 0 {
                        print!("{}", local_ui_features[tile.features.last().expect("(X) - No features.")]);
                    } else if tile.state {
                        print!("{}", local_ui_tiles.on);
                    } else {
                        print!("{}", local_ui_tiles.off);
                    }
                    found = true;
                }
                i += 1;
            }
            if !found {
                print!("{}", local_ui_tiles.void);
            }
        }
        println!();
    }
}

