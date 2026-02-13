use bevy::prelude::*;

use falling_sand::FallingSand;

fn main() {
    match App::new().add_plugins(FallingSand).run() {
        AppExit::Success => info!("good bye"),
        AppExit::Error(n) => error!("exited with error code: {n}"),
    }
}
