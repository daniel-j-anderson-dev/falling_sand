use bevy::prelude::*;

use crate::game_of_life::*;

pub mod startup {

    use super::*;

    pub fn setup_camera(mut commands: Commands) {
        debug!("spawning camera");
        commands.spawn(Camera2d);
    }

    pub fn spawn_grid(
        mut commands: Commands,
        conf: Res<GridConfiguration>,
        mut grid_map: ResMut<GridMap>,
    ) {
        debug!("spawning ({} x {}) grid", conf.height, conf.width);

        grid_map.cells = GridCoordinate::all(&conf)
            .map(|entity_pos| {
                let entity = commands
                    .spawn((
                        Cell,
                        CellState { alive: false },
                        NextCellState { alive: false },
                        entity_pos,
                        Sprite {
                            color: Color::srgb(0.2, 0.2, 0.2),
                            custom_size: Some(Vec2::splat(conf.cell_size - 1.0)),
                            ..default()
                        },
                        Transform::from_xyz(
                            (entity_pos.x as f32 - conf.width as f32 / 2.0) * conf.cell_size,
                            (entity_pos.y as f32 - conf.height as f32 / 2.0) * conf.cell_size,
                            0.0,
                        ),
                    ))
                    .id();

                (entity_pos, entity)
            })
            .collect();
    }

    pub fn initialize_random_cells(mut query: Query<&mut CellState, With<Cell>>) {
        use rand::RngExt;
        let mut rng = rand::rng();
        for mut state in query.iter_mut() {
            state.alive = rng.random_bool(0.3);
        }
        debug!("all cells randomized");
    }
}

pub mod update {
    use super::*;

    pub fn update_cell_color_on_state_change(
        mut query: Query<(&CellState, &mut Sprite), (With<Cell>, Changed<CellState>)>,
    ) {
        for (state, mut sprite) in query.iter_mut() {
            sprite.color = if state.alive {
                Color::srgb(1.0, 1.0, 1.0)
            } else {
                Color::srgb(0.2, 0.2, 0.2)
            };
        }
    }

    pub fn calculate_next_generation(
        timer: Res<UpdateTimer>,
        grid_map: Res<GridMap>,
        mut next_cells: Query<(&CellState, &GridCoordinate, &mut NextCellState), With<Cell>>,
        current_cells: Query<&CellState, With<Cell>>,
    ) {
        if !timer.just_finished() {
            return;
        }

        for (current_state, current_pos, mut next_state) in next_cells.iter_mut() {
            let neighbor_count = current_pos
                .neighbors()
                .filter(|neighbor_pos| {
                    grid_map
                        .cells
                        .get(neighbor_pos)
                        .and_then(|&neighbor_entity| {
                            current_cells
                                .get(neighbor_entity)
                                .ok()
                                .map(|neighbor_state| neighbor_state.alive)
                        })
                        .unwrap_or(false)
                })
                .count();
            next_state.alive = matches!(
                (current_state.alive, neighbor_count),
                (true, 2) | (true, 3) | (false, 3)
            );
        }
    }

    pub fn apply_next_generation(
        mut query: Query<(&mut CellState, &NextCellState), With<Cell>>, //
        timer: Res<UpdateTimer>,
        mut generation_count: ResMut<GenerationCount>,
    ) {
        if !timer.just_finished() {
            return;
        }

        for (mut current, next) in query.iter_mut() {
            current.alive = next.alive;
        }

        info!("end of generation {}", generation_count.0);
        generation_count.0 += 1;
    }

    pub fn tick_update_timer(time: Res<Time>, mut timer: ResMut<UpdateTimer>) {
        timer.0.tick(time.delta());
    }
}
