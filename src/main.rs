use bevy::prelude::*;

struct Config {
    grid_size: IVec2,
    cell_size: IVec2,
}

struct Coords(IVec2);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Config {
            grid_size: IVec2::splat(64),
            cell_size: IVec2::splat(10),
        })
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, config: Res<Config>) {
    let material = materials.add(Color::CRIMSON.into());
    
    let width = config.grid_size.x / 2;
    let height = config.grid_size.y / 2;

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    for y in -height..=height {
        for x in -width..=width {
            let coords = Coords(IVec2::new(x, y));
            let translation = config.cell_size.extend(0).as_f32() * coords.0.extend(0).as_f32();
            let transform = Transform::from_translation(translation);

            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    transform,
                    material: material.clone(),
                    sprite: Sprite::new(config.cell_size.as_f32() - Vec2::new(2.0, 2.0)),
                    ..Default::default()
                })
                .insert(coords);
        }
    }
}
