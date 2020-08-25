mod life;
mod board;

use bevy::prelude::*;
use life::Life;

fn main() {
    App::build().add_default_plugins().add_plugin(Life).run();
}
