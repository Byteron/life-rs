use bevy::{core::FixedTimestep, prelude::*};
use rand::Rng;

use super::board::*;

#[derive(Copy, Clone)]
pub struct Life {
    pub board_width: i32,
    pub board_height: i32,
    pub pixel_size: Vec2,
    pub border_size: Vec2,
    pub board_color: Color,
    pub alive_color: Color,
    pub dead_color: Color,
}

impl Plugin for Life {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.clone())
            .add_resource(Board::new(self.board_width, self.board_height))
            .add_stage_after(
                stage::UPDATE,
                "fixed_update",
                SystemStage::serial()
                    .with_run_criteria(FixedTimestep::step(0.050))
                    .with_system(rules.system())
                    .with_system(update_tiles.system()),
            )
            .add_startup_system(setup.system());
    }
}

impl Life {
    fn get_tile_pixel_size(&self) -> Vec2 {
        self.pixel_size / Vec2::new(self.board_width as f32, self.board_height as f32)
    }

    fn get_board_pixel_center(&self) -> Vec2 {
        self.pixel_size / 2.0
    }

    fn get_tile_pixel_offset(&self) -> Vec2 {
        self.get_tile_pixel_size() / 2.0
    }

    fn get_coord_transform(&self, coords: Coordinates) -> Transform {
        let pos2 = coords.to_vec() * self.get_tile_pixel_size() - self.get_board_pixel_center()
            + self.get_tile_pixel_offset();
        let pos3 = Vec3::new(pos2.x, pos2.y, 1.0);
        Transform::from_translation(pos3)
    }
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board: ResMut<Board>,
    life: Res<Life>,
) {
    let theme = Theme {
        border_size: life.border_size,
        board_mat: materials.add(life.board_color.into()),
        alive_mat: materials.add(life.alive_color.into()),
        dead_mat: materials.add(life.dead_color.into()),
    };

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: theme.get_board_mat(),
            transform: Transform::from_translation(Vec3::zero()),
            sprite: Sprite {
                size: life.pixel_size,
                resize_mode: SpriteResizeMode::default(),
            },
            ..Default::default()
        });

    let tile_pixel_size = life.get_tile_pixel_size();
    let mut rng = rand::thread_rng();

    for y in 0..board.height {
        for x in 0..board.width {
            let coords = Coordinates::new(x, y);

            let state = if rng.gen_bool(0.5) {
                TileState::Alive
            } else {
                TileState::Dead
            };

            let material = match state {
                TileState::Alive => theme.get_alive_mat(),
                TileState::Dead => theme.get_dead_mat(),
            };

            commands
                .spawn(SpriteBundle {
                    material: material,
                    transform: life.get_coord_transform(coords),
                    sprite: Sprite {
                        size: tile_pixel_size - theme.border_size,
                        resize_mode: SpriteResizeMode::default(),
                    },
                    ..Default::default()
                })
                .with(coords)
                .with(Generation { state });

            board.tiles.insert(coords, Tile { state });
        }
    }

    commands.insert_resource(theme);
}

fn rules(board: ResMut<Board>, mut query: Query<(&Coordinates, &mut Generation)>) {
    for (coords, mut gen) in query.iter_mut() {
        let tile = board.get_tile(coords).unwrap();

        let mut alive_count = 0;

        for n_coords in coords.get_neighbors().iter() {
            if let Some(n_tile) = board.get_tile(n_coords) {
                match n_tile.state {
                    TileState::Alive => {
                        alive_count += 1;
                    }
                    TileState::Dead => {}
                }
            }
        }

        match tile.state {
            TileState::Alive => {
                if alive_count > 3 || alive_count < 2 {
                    gen.state = TileState::Dead;
                }
            }
            TileState::Dead => {
                if alive_count == 3 {
                    gen.state = TileState::Alive;
                }
            }
        }
    }
}

fn update_tiles(
    mut board: ResMut<Board>,
    theme: Res<Theme>,
    mut query: Query<(&Coordinates, &Generation, &mut Handle<ColorMaterial>), Changed<Generation>>,
) {
    for (coords, gen, mut mat) in query.iter_mut() {
        let mut tile = board.get_mut_tile(coords).unwrap();
        tile.state = gen.state;

        match tile.state {
            TileState::Alive => {
                *mat = theme.get_alive_mat();
            }
            TileState::Dead => {
                *mat = theme.get_dead_mat();
            }
        }
    }
}
