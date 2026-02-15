use bevy::prelude::*;

use crate::game_of_life::*;

#[derive(Component)]
pub struct Cell;

#[derive(Component)]
pub struct CellState {
    pub alive: bool,
}

#[derive(Component)]
pub struct NextCellState {
    pub alive: bool,
}

#[derive(Component, Hash, PartialEq, Eq, Clone, Copy)]
pub struct GridCoordinate {
    pub x: i32,
    pub y: i32,
}
impl GridCoordinate {
    pub fn all(
        &GridConfiguration { height, width, .. }: &GridConfiguration,
    ) -> impl Iterator<Item = Self> {
        (0..height).flat_map(move |x| (0..width).map(move |y| Self { x, y }))
    }
    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(move |x_offset| {
            (-1..=1)
                .filter(move |&y_offset| x_offset != 0 || y_offset != 0)
                .map(move |y_offset| Self {
                    x: self.x + x_offset,
                    y: self.y + y_offset,
                })
        })
    }
}
