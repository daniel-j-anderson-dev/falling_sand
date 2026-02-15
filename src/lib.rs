use bevy::prelude::*;

pub mod game_of_life;

pub struct FallingSand;
impl Plugin for FallingSand {
    fn build(&self, app: &mut App) {
        app //
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    hello_world, //
                ),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("setup complete");
}

fn hello_world() {
    info!("hello world!")
}
