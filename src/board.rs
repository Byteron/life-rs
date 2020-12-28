use bevy::{math::Vec2, prelude::Handle, sprite::ColorMaterial};
use std::{collections::HashMap, ops::Add};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum TileState {
    Alive,
    Dead,
}

pub struct Tile {
    pub state: TileState,
}

pub struct Generation {
    pub state: TileState,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub tiles: HashMap<Coordinates, Tile>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        Board {
            width,
            height,
            tiles: HashMap::default(),
        }
    }

    pub fn get_tile(&self, coords: &Coordinates) -> Option<&Tile> {
        self.tiles.get(coords)
    }

    pub fn get_mut_tile(&mut self, coords: &Coordinates) -> Option<&mut Tile> {
        self.tiles.get_mut(coords)
    }
}

#[derive(Default)]
pub struct Theme {
    pub border_size: Vec2,
    pub board_mat: Handle<ColorMaterial>,
    pub alive_mat: Handle<ColorMaterial>,
    pub dead_mat: Handle<ColorMaterial>,
}

impl Theme {
    pub fn get_board_mat(&self) -> Handle<ColorMaterial> {
        self.board_mat.clone()
    }

    pub fn get_alive_mat(&self) -> Handle<ColorMaterial> {
        self.alive_mat.clone()
    }

    pub fn get_dead_mat(&self) -> Handle<ColorMaterial> {
        self.dead_mat.clone()
    }
}
