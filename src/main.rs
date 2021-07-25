use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

// RESOURCES

struct Config {
    width: i32,
    height: i32,
    board_color: Color,
    alive_color: Color,
    dead_color: Color,
}

pub struct Theme {
    pub board_mat: Handle<ColorMaterial>,
    pub alive_mat: Handle<ColorMaterial>,
    pub dead_mat: Handle<ColorMaterial>,
}

struct Cells(HashMap<Coords, Entity>);

// COMPONENTS

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coords(IVec2);

impl Coords {
    fn get_neighbors(&self) -> [Coords; 8] {
        [
            Coords(self.0 + IVec2::new(-1, -1)),
            Coords(self.0 + IVec2::new(0, -1)),
            Coords(self.0 + IVec2::new(1, -1)),
            Coords(self.0 + IVec2::new(-1, 0)),
            Coords(self.0 + IVec2::new(1, 0)),
            Coords(self.0 + IVec2::new(-1, 1)),
            Coords(self.0 + IVec2::new(0, 1)),
            Coords(self.0 + IVec2::new(1, 1)),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Alive,
    Dead,
}

struct StarvedEvent {
    entity: Entity,
}
struct RevivedEvent {
    entity: Entity,
}

// MAIN

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<RevivedEvent>()
        .add_event::<StarvedEvent>()
        .insert_resource(Config {
            width: 128,
            height: 128,
            board_color: Color::rgb(0.2, 0.2, 0.2),
            alive_color: Color::rgb(0.8, 0.8, 0.8),
            dead_color: Color::rgb(0.1, 0.1, 0.1),
        })
        .add_startup_system(setup)
        .add_system(tick.label("Tick"))
        .add_system(revive.after("Tick"))
        .add_system(starve.after("Tick"))
        .run();
}

// SYSTEMS

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    config: Res<Config>,
) {
    let theme = Theme {
        board_mat: materials.add(config.board_color.into()),
        alive_mat: materials.add(config.alive_color.into()),
        dead_mat: materials.add(config.dead_color.into()),
    };

    let mut cells = Cells(HashMap::default());

    let screen_height = windows.get_primary().unwrap().height();
    let cell_size = Vec2::splat(screen_height / config.height as f32);
    let offset = cell_size.extend(0.0) / 2.0;

    let center_cell = Vec2::new(config.width as f32, config.height as f32) / 2.0 * cell_size;

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_translation(center_cell.extend(999.9)));

    commands.spawn().insert_bundle(SpriteBundle {
        transform: Transform::from_translation(center_cell.extend(0.0)),
        material: theme.board_mat.clone(),
        sprite: Sprite::new(Vec2::splat(screen_height as f32) + Vec2::splat(2.0)),
        ..Default::default()
    });

    let mut rng = rand::thread_rng();

    for y in 0..config.height {
        for x in 0..config.width {
            let cell = IVec2::new(x, y);
            let translation = cell_size.extend(0.0) * cell.extend(0).as_f32() + offset;
            let transform = Transform::from_translation(translation);

            let state;
            let material;

            if rng.gen_bool(0.5) {
                state = State::Alive;
                material = theme.alive_mat.clone();
            } else {
                state = State::Dead;
                material = theme.dead_mat.clone();
            }
            let entity = commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    transform,
                    material,
                    sprite: Sprite::new(cell_size - Vec2::splat(2.0)),
                    ..Default::default()
                })
                .insert(state)
                .insert(Coords(cell))
                .id();

            cells.0.insert(Coords(cell), entity);
        }
    }

    commands.insert_resource(theme);
    commands.insert_resource(cells);
}

fn tick(
    mut starved_writer: EventWriter<StarvedEvent>,
    mut revived_writer: EventWriter<RevivedEvent>,
    cells: Res<Cells>,
    query: Query<&State>,
) {
    for (coords, entity) in cells.0.iter() {
        let mut alive_count = 0;

        for n_coords in coords.get_neighbors().iter() {
            if let Some(n_entity) = cells.0.get(n_coords) {
                if let Ok(state) = query.get(*n_entity) {
                    if *state == State::Alive {
                        alive_count += 1;
                    }
                }
            }
        }

        match query.get(*entity).unwrap() {
            State::Alive => {
                if alive_count > 3 || alive_count < 2 {
                    starved_writer.send(StarvedEvent { entity: *entity });
                }
            }
            State::Dead => {
                if alive_count == 3 {
                    revived_writer.send(RevivedEvent { entity: *entity });
                }
            }
        }
    }
}

fn revive(
    theme: Res<Theme>,
    mut reader: EventReader<RevivedEvent>,
    mut query: Query<(&mut State, &mut Handle<ColorMaterial>)>,
) {
    for event in reader.iter() {
        if let Ok((mut state, mut mat)) = query.get_mut(event.entity) {
            *state = State::Alive;
            *mat = theme.alive_mat.clone();
        }
    }
}

fn starve(
    theme: Res<Theme>,
    mut reader: EventReader<StarvedEvent>,
    mut query: Query<(&mut State, &mut Handle<ColorMaterial>)>,
) {
    for event in reader.iter() {
        if let Ok((mut state, mut mat)) = query.get_mut(event.entity) {
            *state = State::Dead;
            *mat = theme.dead_mat.clone();
        }
    }
}
