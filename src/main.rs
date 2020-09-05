mod board;
mod life;

use bevy::{diagnostic::*, prelude::*};
use life::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(Life {
            width: 80,
            height: 80,
            border: Vec2::new(2.0, 2.0),
            color_board: Color::rgb(0.2, 0.2, 0.2),
            color_alive: Color::rgb(0.8, 0.8, 0.8),
            color_dead: Color::rgb(0.1, 0.1, 0.1),
        })
        .run();
}
