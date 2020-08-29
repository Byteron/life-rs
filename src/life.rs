use bevy::prelude::*;
use rand::Rng;

use super::board::*;

#[derive(Copy, Clone)]
pub struct Life {
    pub width: i32,
    pub height: i32,
    pub border: Vec2,
    pub color_board: Color,
    pub color_alive: Color,
    pub color_dead: Color,
}

impl Plugin for Life {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.clone())
            .add_resource(Board::new(self.width, self.height))
            .add_startup_system(setup.system())
            .add_system(rules.system())
            .add_system_to_stage(stage::POST_UPDATE, update_tiles.system());
    }
}

#[derive(Default)]
struct Theme {
    border: Vec2,
    board: Handle<ColorMaterial>,
    alive: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board: ResMut<Board>,
    life: Res<Life>,
) {
    let theme = Theme {
        border: life.border,
        board: materials.add(life.color_board.into()),
        alive: materials.add(life.color_alive.into()),
        dead: materials.add(life.color_dead.into()),
    };

    let pixel_size = Vec2::new(600.0, 600.0);
    let tile_size = pixel_size / board.size();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: theme.board,
            translation: Translation(Vec3::zero()),
            sprite: Sprite { size: pixel_size },
            ..Default::default()
        });

    for idx in 0..board.len() {
        let mut rng = rand::thread_rng();
        
        let coords = board.idx2cds(idx);

        let offset = tile_size / Vec2::new(2.0, 2.0);
        let center = pixel_size / Vec2::new(2.0, 2.0);

        let pos2 = board.idx2cds(idx).to_vec() * tile_size - center + offset;
        let pos3 = Vec3::new(pos2.x(), pos2.y(), 1.0);

        let state = if rng.gen_bool(0.5) {
            State::Alive
        } else {
            State::Dead
        };

        commands
            .spawn(Camera2dComponents::default())
            .spawn(SpriteComponents {
                material: theme.alive,
                translation: Translation(pos3),
                sprite: Sprite {
                    size: tile_size - theme.border,
                },
                ..Default::default()
            })
            .with(Generation::new(state))
            .with(coords);


        board.tiles.push(Tile::new(state, coords.get_neighbors()));
    }

    commands.insert_resource(theme);
}

fn rules(board: ResMut<Board>, mut query: Query<(&Coordinates, &mut Generation)>) {
    for (coords, mut gen) in &mut query.iter() {
        let tile = board.get_tile(*coords).unwrap();

        let alive_count = tile
            .neighbors
            .iter()
            .filter(|n| match board.get_tile(**n) {
                Some(tile) => {
                    if tile.state == State::Alive {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            })
            .count();

        match tile.state {
            State::Alive => {
                if alive_count < 2 || alive_count > 3 {
                    gen.state = State::Dead;
                }
            }
            State::Dead => {
                if alive_count == 3 {
                    gen.state = State::Alive;
                }
            }
        }
    }
}

fn update_tiles(
    mut board: ResMut<Board>,
    colors: Res<Theme>,
    mut query: Query<(
        &Coordinates,
        &mut Generation,
        &mut Handle<ColorMaterial>,
    )>,
) {
    for (coords, gen, mut mat) in &mut query.iter() {
        let mut tile = board.get_mut_tile(*coords).unwrap();
        tile.state = gen.state;

        match tile.state {
            State::Alive => {
                *mat = colors.alive;
            }
            State::Dead => {
                *mat = colors.dead;
            }
        }
    }
}
