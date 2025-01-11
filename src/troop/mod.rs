pub struct Troop {
    pos: (usize, usize),
    dir: (i8, i8),
    name: TroopType,
    health: u8,
    attack: i8,
    defense: u8,
    range: u8,
    mobility: u8,
}

pub enum TroopType {
    Spearman,
    Archer
}

impl Troop {
    pub fn new(name: TroopType, pos: (usize, usize)) -> Troop {
        match name {
            TroopType::Spearman => Troop {
                pos: pos,
                dir: (1, 0),
                name: name,
                health: 100,
                attack: 20,
                defense: 10,
                range: 2,
                mobility: 1,
            },
            TroopType::Archer => Troop {
                pos: pos,
                dir: (1, 0),
                name: name,
                health: 50,
                attack: 10,
                defense: 5,
                range: 20,
                mobility: 1,
            },
        }
    }
}
