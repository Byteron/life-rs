mod board;
mod life;

use bevy::{diagnostic::*, prelude::*};
use life::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(Life {
            board_width: 80,
            board_height: 80,
            pixel_size: Vec2::new(700.0, 700.0),
            border_size: Vec2::new(2.0, 2.0),
            board_color: Color::rgb(0.2, 0.2, 0.2),
            alive_color: Color::rgb(0.8, 0.8, 0.8),
            dead_color: Color::rgb(0.1, 0.1, 0.1),
        })
        .run();
}
