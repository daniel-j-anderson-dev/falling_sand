use bevy::prelude::*;

use falling_sand::{FallingSand, game_of_life::GameOfLife};

fn main() {
    match App::new()
        .add_plugins(GameOfLife) //
        // .add_plugins(FallingSand) //
        .run()
    {
        AppExit::Success => info!("good bye"),
        AppExit::Error(n) => error!("exited with error code: {n}"),
    }
}
