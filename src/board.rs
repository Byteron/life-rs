use bevy::{ecs::Entity, math::Vec2, prelude::Handle, sprite::ColorMaterial};
use std::{ops::Add};

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
    pub tiles: Vec<Vec<Option<Entity>>>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = Vec::default();
        
        tiles.append(&mut vec![Vec::default(); width as usize]);

        for column in tiles.iter_mut() {
            column.append(&mut vec![None; height as usize]);
        }
        
        Board {
            width,
            height,
            tiles,
        }
    }

    pub fn set(&mut self, coords: Coordinates, entity: Entity) {
        self.tiles[coords.x as usize][coords.y as usize] = Some(entity);
    }

    pub fn get(&self, coords: &Coordinates) -> Option<Entity> {
        if let Some(column) = self.tiles.get(coords.y as usize) {
            if let Some(entity) = column.get(coords.x as usize) {
                return *entity;
            }
        }

        return None;
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
