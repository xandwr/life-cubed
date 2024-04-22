/// Represents a cell in the world grid.
#[derive(Clone)]
pub struct Cell {
    pub position: [i32; 3],
    pub is_alive: bool,
    pub color: [f32; 3]
}