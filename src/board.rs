use bevy::math::Vec2;


#[derive(Copy, Clone, Debug)]
pub enum TileState {
    Alive,
    Dead,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub position: Vec2,
    pub state: TileState
}

impl Tile {
    fn new() -> Self {
        Tile { position: Vec2::zero(), state: TileState::Alive }
    }


}
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub tiles: Box<Vec<Tile>>,
    pub border: Vec2,
}

impl Board {
    pub fn new(width: i32, height: i32, border: f32) -> Self {
        Board {
            width,
            height,
            tiles: Box::new(vec![Tile::new(); (width * height) as usize]),
            border: Vec2::new(border, border)
        }
    }

    pub fn length(&self) -> i32 {
        self.width * self.height
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn idx2vec(&self, index: i32) -> Vec2 {
        let x = (index % self.width) as f32;
        let y = (index / self.width) as f32;

        Vec2::new(x, y)
    }
}