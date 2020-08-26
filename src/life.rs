use super::board::{Board, Tile, TileState};
use bevy::prelude::*;
use rand::Rng;

pub struct Life;

impl Plugin for Life {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Board::new(20, 20, 2.0))
            .add_resource(ColorTheme {
                board: Color::rgb(0.2, 0.2, 0.2),
                alive: Color::rgb(0.8, 0.8, 0.8),
                dead: Color::rgb(0.1, 0.1, 0.1),
            })
            .add_startup_system(setup.system())
            .add_system(draw_tiles.system());
    }
}

struct ColorTheme {
    board: Color,
    alive: Color,
    dead: Color,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    color_theme: Res<ColorTheme>,
    mut board: ResMut<Board>,
) {
    let mut rand = rand::thread_rng();
    let pixel_size = Vec2::new(600.0, 600.0);
    let tile_size = pixel_size / board.size();

    let sprite = commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(color_theme.board.into()),
            translation: Translation(Vec3::zero()),
            sprite: Sprite {
                size: pixel_size + board.border,
            },
            ..Default::default()
        });

    for i in 0..board.length() {
        if rand.gen_bool(0.5) {
            board.tiles[i as usize].state = TileState::Dead;
        }

        let offset = tile_size / Vec2::new(2.0, 2.0);
        let center = pixel_size / Vec2::new(2.0, 2.0);

        let pos2 = board.idx2vec(i) * tile_size - center + offset;
        let pos3 = Vec3::new(pos2.x(), pos2.y(), 1.0);

        let sprite = SpriteComponents {
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            translation: Translation(pos3),
            sprite: Sprite {
                size: tile_size - board.border,
            },
            ..Default::default()
        };

        let tile = board.tiles[i as usize];

        commands.spawn(sprite).with(tile);
    }
}

fn draw_tiles(
    color_theme: Res<ColorTheme>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Tile, &mut Handle<ColorMaterial>)>
) {
    for (tile, color) in &mut query.iter() {
        match tile.state {
            TileState::Alive => {
                let mut color_mat = materials.get_mut(&color).unwrap();
                color_mat.color = color_theme.alive;
            }
            TileState::Dead => {
                let mut color_mat = materials.get_mut(&color).unwrap();
                color_mat.color = color_theme.dead;
            }
        }
    }
}
