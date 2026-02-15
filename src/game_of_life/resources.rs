use std::collections::HashMap;

use bevy::prelude::*;

use crate::game_of_life::components::GridCoordinate;

#[derive(Resource)]
pub struct GridConfiguration {
    pub height: i32,
    pub width: i32,
    pub cell_size: f32,
}

#[derive(Resource, Deref, DerefMut)]
pub struct UpdateTimer(pub Timer);

#[derive(Default, Resource)]
pub struct GridMap {
    pub cells: HashMap<GridCoordinate, Entity>,
}


#[derive(Resource, Default)]
pub struct GenerationCount(pub usize);