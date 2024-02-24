use crate::{BRICK_TILE, CONCRETE_TILE, EMPTY_FRAME_TILE, NET_TILE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WallType {
    Brick,
    Concrete,
    Net,
    Empty,
}

pub struct Wall {
    variant: WallType,
    position: [i32; 2],
}

impl Wall {
    pub fn new(position: [i32; 2]) -> Wall {
        Wall {
            variant: WallType::Empty,
            position,
        }
    }

    pub fn variant(&self) -> WallType {
        self.variant
    }

    pub fn brick(mut self) -> Self {
        self.variant = WallType::Brick;
        self
    }

    pub fn concrete(mut self) -> Self {
        self.variant = WallType::Concrete;
        self
    }

    pub fn net(mut self) -> Self {
        self.variant = WallType::Net;
        self
    }

    pub fn empty(mut self) -> Self {
        self.variant = WallType::Empty;
        self
    }

    pub fn damage(&mut self) {
        match self.variant {
            WallType::Brick => self.variant = WallType::Empty,
            WallType::Concrete => self.variant = WallType::Concrete,
            WallType::Net => self.variant = WallType::Net,
            WallType::Empty => self.variant = WallType::Empty,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self.variant {
            WallType::Brick | WallType::Concrete => true,
            WallType::Net | WallType::Empty => false,
        }
    }

    pub fn get_frame(&self) -> &[f64; 4] {
        match self.variant {
            WallType::Brick => &BRICK_TILE,
            WallType::Concrete => &CONCRETE_TILE,
            WallType::Net => &NET_TILE,
            WallType::Empty => &EMPTY_FRAME_TILE,
        }
    }

    pub fn get_position(&self) -> &[i32; 2] {
        &self.position
    }
}

pub fn generate_walls(column_count: u8, row_count: u8) -> Vec<Vec<Wall>> {
    let mut walls = vec![];

    for x in 0..row_count {
        let mut row = vec![];

        for y in 0..column_count {
            let rng = rand::random::<u8>() % 6;

            match rng {
                0 | 1 => row.push(Wall::new([x.into(), y.into()]).brick()),
                2 => row.push(Wall::new([x.into(), y.into()]).concrete()),
                3 => row.push(Wall::new([x.into(), y.into()]).net()),
                _ => row.push(Wall::new([x.into(), y.into()]).empty()),
            }
        }

        walls.push(row);
    }

    [
        [0, 0],
        [row_count as i32 - 1, 0],
        [0, column_count as i32 - 1],
        [row_count as i32 - 1, column_count as i32 - 1],
    ]
    .iter()
    .for_each(|&[x, y]| {
        walls[x as usize][y as usize] = Wall::new([x, y]).empty();
    });

    walls
}
