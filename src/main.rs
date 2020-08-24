use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
enum TileType {
    Alive,
    Dead
}

struct Board {
    width: i32,
    height: i32,
    tiles: Box<Vec<TileType>>,
}

impl Board {
    fn new(width: i32, height: i32) -> Self {
        Board {
            width,
            height,
            tiles: Box::new(vec![TileType::Dead; (width * height) as usize]),
        }
    }
}

struct Life;

impl Plugin for Life {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Board::new(8, 8))
            .add_startup_system(setup.system());
    }
}

fn setup(mut board: ResMut<Board>) {
    println!("Board Size: ({}, {})", board.width, board.height);
    println!("Tiles: {:?})", board.tiles);
}

fn main() {
    App::build().add_default_plugins().add_plugin(Life).run();
}
