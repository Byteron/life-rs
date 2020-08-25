use super::board::{Board, TileState};
use bevy::prelude::*;
use rand::Rng;

pub struct Life;

impl Plugin for Life {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Board::new(64, 64, 2.0))
            .add_startup_system(setup.system())
            .add_resource(ColorTheme {
                board: Color::rgb(0.2, 0.2, 0.2),
                alive: Color::rgb(0.8, 0.8, 0.8),
                dead: Color::rgb(0.1, 0.1, 0.1),
            });
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

    commands
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
        let alive = rand.gen_bool(0.5);
        let color: Color;

        if alive {
            board.tiles[i as usize].state = TileState::Alive;
            color = color_theme.alive;
        } else {
            board.tiles[i as usize].state = TileState::Dead;
            color = color_theme.dead;
        }

        let pos2 = board.idx2vec(i) * tile_size - pixel_size / Vec2::new(2.0, 2.0)
            + tile_size / Vec2::new(2.0, 2.0);
        let pos3 = Vec3::new(pos2.x(), pos2.y(), 1.0);

        commands.spawn(SpriteComponents {
            material: materials.add(color.into()),
            translation: Translation(pos3),
            sprite: Sprite {
                size: tile_size - board.border,
            },
            ..Default::default()
        });
    }
}
