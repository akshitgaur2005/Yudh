pub struct Grid {
    field: Vec<Vec<Cell>>,
}

#[derive(Clone)]
pub struct Cell {}

impl Grid {
    pub fn new(x: usize, y: usize) -> Grid {
        let field = vec![vec![Cell {}; x]; y];

        Grid { field }
    }
}
