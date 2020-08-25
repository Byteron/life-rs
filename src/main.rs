mod board;
mod life;

use bevy::prelude::*;
use life::Life;

fn main() {
    App::build().add_default_plugins().add_plugin(Life).run();
}
