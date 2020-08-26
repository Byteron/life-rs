use bevy::math::Vec2;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileState {
    Alive,
    Dead,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub position: Vec2,
    pub state: TileState,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            position: Vec2::zero(),
            state: TileState::Alive,
        }
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
        let mut board = Board {
            width,
            height,
            tiles: Box::new(vec![Tile::new(); (width * height) as usize]),
            border: Vec2::new(border, border),
        };

        for i in 0..board.length() {
            board.tiles[i as usize].position = board.idx2vec(i);
        }

        board
    }

    pub fn length(&self) -> i32 {
        self.width * self.height
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn get_neighbors(&self, position: Vec2) -> Vec<Tile> {
        let mut neighbors = Vec::new();

        let mut positions: Vec<Vec2> = Vec::new();

        positions.push(position + Vec2::new(0.0, 1.0));
        positions.push(position + Vec2::new(0.0, -1.0));
        positions.push(position + Vec2::new(1.0, 0.0));
        positions.push(position + Vec2::new(-1.0, 0.0));

        positions.push(position + Vec2::new(1.0, 1.0));
        positions.push(position + Vec2::new(1.0, -1.0));
        positions.push(position + Vec2::new(-1.0, 1.0));
        positions.push(position + Vec2::new(-1.0, -1.0));

        for pos in positions.iter() {
            if pos.x() as i32 >= self.width
                || pos.x() < 0.0
                || pos.y() as i32 >= self.height
                || pos.y() < 0.0
            {
                continue;
            }

            let idx = self.vec2idx(pos.clone()) as usize;
            let tile = self.tiles[idx].clone();
            neighbors.push(tile);
        }

        neighbors
    }

    pub fn idx2vec(&self, index: i32) -> Vec2 {
        let x = (index % self.width) as f32;
        let y = (index / self.width) as f32;

        Vec2::new(x, y)
    }

    pub fn vec2idx(&self, vec: Vec2) -> i32 {
        vec.y() as i32 * self.width + vec.x() as i32
    }
}
