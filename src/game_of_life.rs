use bevy::prelude::*;

use self::{
    components::*,
    resources::*,
    systems::{startup::*, update::*},
};

mod components;
mod resources;
mod systems;

pub struct GameOfLife;
impl Plugin for GameOfLife {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .init_resource::<GridMap>()
            .init_resource::<GenerationCount>()
            .insert_resource(GridConfiguration {
                height: 100,
                width: 100,
                cell_size: 10.0,
            })
            .insert_resource(UpdateTimer(Timer::from_seconds(
                0.25,
                TimerMode::Repeating, //
            )))
            .add_systems(
                Startup,
                (
                    setup_camera, //
                    spawn_grid,
                    initialize_random_cells,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    calculate_next_generation, //
                    apply_next_generation,
                    update_cell_color_on_state_change,
                    tick_update_timer,
                )
                    .chain(),
            );
    }
}
