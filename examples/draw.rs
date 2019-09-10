use grid_2d::{Coord, Grid, Size};
use static_circle::circle_with_squared_radius;

circle_with_squared_radius!(200, COORDS, NUM_COORDS, grid_2d::Coord);

pub fn main() {
    println!("{}", NUM_COORDS);
    let mut grid = Grid::new_clone(Size::new(40, 40), ' ');
    let offset = Coord::new(20, 20);
    for &coord in COORDS.iter() {
        if let Some(cell) = grid.get_mut(coord + offset) {
            *cell = '#';
        }
    }
    for row in grid.rows() {
        for ch in row {
            print!("{}", ch);
        }
        println!("");
    }
}
