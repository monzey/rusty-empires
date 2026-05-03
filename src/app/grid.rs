use bevy::{prelude::*, window::PrimaryWindow};

use super::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use crate::GridPosition;

pub(super) fn cursor_grid_position(
    windows: &Query<&Window, With<PrimaryWindow>>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<GridPosition> {
    let window = windows.get_single().ok()?;
    let cursor_position = window.cursor_position()?;
    let (camera, camera_transform) = cameras.get_single().ok()?;
    let world_position = camera
        .viewport_to_world(camera_transform, cursor_position)?
        .origin
        .truncate();

    world_to_grid(world_position)
}

pub(super) fn grid_to_world(position: GridPosition, z: f32) -> Vec3 {
    let map_width = MAP_WIDTH as f32 * TILE_SIZE;
    let map_height = MAP_HEIGHT as f32 * TILE_SIZE;

    Vec3::new(
        (position.x as f32 + 0.5) * TILE_SIZE - map_width / 2.0,
        (position.y as f32 + 0.5) * TILE_SIZE - map_height / 2.0,
        z,
    )
}

fn world_to_grid(position: Vec2) -> Option<GridPosition> {
    let map_width = MAP_WIDTH as f32 * TILE_SIZE;
    let map_height = MAP_HEIGHT as f32 * TILE_SIZE;
    let x = ((position.x + map_width / 2.0) / TILE_SIZE).floor() as i32;
    let y = ((position.y + map_height / 2.0) / TILE_SIZE).floor() as i32;
    let grid_position = GridPosition { x, y };

    is_inside_map(grid_position).then_some(grid_position)
}

pub(super) fn is_inside_map(position: GridPosition) -> bool {
    position.x >= 0 && position.x < MAP_WIDTH && position.y >= 0 && position.y < MAP_HEIGHT
}
