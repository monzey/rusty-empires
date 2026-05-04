use bevy::{prelude::*, window::PrimaryWindow};

use super::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_HEIGHT, TILE_WIDTH};
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
    let tile_x = position.x as f32 + 0.5;
    let tile_y = position.y as f32 + 0.5;
    let raw_x = (tile_x - tile_y) * TILE_WIDTH / 2.0;
    let raw_y = -(tile_x + tile_y) * TILE_HEIGHT / 2.0;
    let center_x = (MAP_WIDTH as f32 - MAP_HEIGHT as f32) * TILE_WIDTH / 4.0;
    let center_y = -(MAP_WIDTH as f32 + MAP_HEIGHT as f32) * TILE_HEIGHT / 4.0;
    let draw_order = (position.x + position.y) as f32 * 0.01;

    Vec3::new(raw_x - center_x, raw_y - center_y, z + draw_order)
}

fn world_to_grid(position: Vec2) -> Option<GridPosition> {
    let center_x = (MAP_WIDTH as f32 - MAP_HEIGHT as f32) * TILE_WIDTH / 4.0;
    let center_y = -(MAP_WIDTH as f32 + MAP_HEIGHT as f32) * TILE_HEIGHT / 4.0;
    let raw_x = position.x + center_x;
    let raw_y = position.y + center_y;
    let axis_x = raw_x / (TILE_WIDTH / 2.0);
    let axis_y = -raw_y / (TILE_HEIGHT / 2.0);
    let x = ((axis_x + axis_y) / 2.0).floor() as i32;
    let y = ((axis_y - axis_x) / 2.0).floor() as i32;
    let grid_position = GridPosition { x, y };

    is_inside_map(grid_position).then_some(grid_position)
}

pub(super) fn is_inside_map(position: GridPosition) -> bool {
    position.x >= 0 && position.x < MAP_WIDTH && position.y >= 0 && position.y < MAP_HEIGHT
}
