use bevy::math::Vec2;
use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum TileState {
    Alive,
    Dead,
}

#[derive(Copy, Clone, Debug)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinates { x, y }
    }

    pub fn get_neighbors(&self) -> [Coordinates; 8] {
        [
            *self + Coordinates::new(-1, -1),
            *self + Coordinates::new(0, -1),
            *self + Coordinates::new(1, -1),
            *self + Coordinates::new(-1, 0),
            *self + Coordinates::new(1, 0),
            *self + Coordinates::new(-1, 1),
            *self + Coordinates::new(0, 1),
            *self + Coordinates::new(1, 1),
        ]
    }

    pub fn to_vec(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Tile {
    pub state: TileState,
    pub neighbors: [Coordinates; 8],
}

pub struct Generation {
    pub state: TileState,
}

impl Generation {
    pub fn new(state: TileState) -> Self {
        Generation { state }
    }
}

impl Tile {
    pub fn new(state: TileState, neighbors: [Coordinates; 8]) -> Self {
        Tile { state, neighbors }
    }
}

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        Board {
            width,
            height,
            tiles: Vec::with_capacity((width * height) as usize),
        }
    }

    pub fn get_tile(&self, coords: Coordinates) -> Option<&Tile> {
        self.tiles.get(self.cds2idx(coords) as usize)
    }

    pub fn get_mut_tile(&mut self, coords: Coordinates) -> Option<&mut Tile> {
        let idx = self.cds2idx(coords);
        self.tiles.get_mut(idx as usize)
    }

    pub fn len(&self) -> i32 {
        self.width * self.height
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn idx2cds(&self, idx: i32) -> Coordinates {
        Coordinates {
            x: idx % self.height,
            y: idx / self.height,
        }
    }

    pub fn cds2idx(&self, coords: Coordinates) -> i32 {
        coords.y * self.width + coords.x
    }
}
