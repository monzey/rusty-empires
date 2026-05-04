use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use std::collections::HashSet;

use super::components::{MapPosition, Tile};
use super::constants::{MAP_HEIGHT, MAP_WIDTH};
use super::grid::{grid_to_world, world_to_grid_unbounded};
use super::resources::{ActiveTiles, AppMeshes};
use crate::GridPosition;

const TILE_VIEW_MARGIN: i32 = 6;

pub(super) fn update_tile_viewport(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    app_meshes: Res<AppMeshes>,
    mut active_tiles: ResMut<ActiveTiles>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(visible_tiles) = visible_tile_positions(&windows, &cameras) else {
        return;
    };
    let visible_set: HashSet<GridPosition> = visible_tiles.iter().copied().collect();

    active_tiles.0.retain(|position, entity| {
        let keep = visible_set.contains(position);
        if !keep {
            commands.entity(*entity).despawn();
        }
        keep
    });

    for position in visible_tiles {
        if active_tiles.0.contains_key(&position) {
            continue;
        }

        let entity = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: app_meshes.tile.clone().into(),
                    material: materials.add(Color::srgb(0.01, 0.012, 0.016)),
                    transform: Transform::from_translation(grid_to_world(position, 0.0)),
                    ..default()
                },
                Tile,
                MapPosition(position),
            ))
            .id();
        active_tiles.0.insert(position, entity);
    }
}

fn visible_tile_positions(
    windows: &Query<&Window, With<PrimaryWindow>>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec<GridPosition>> {
    let window = windows.get_single().ok()?;
    let (camera, camera_transform) = cameras.get_single().ok()?;
    let corners = [
        Vec2::new(0.0, 0.0),
        Vec2::new(window.width(), 0.0),
        Vec2::new(0.0, window.height()),
        Vec2::new(window.width(), window.height()),
    ];

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for corner in corners {
        let world_position = camera
            .viewport_to_world(camera_transform, corner)?
            .origin
            .truncate();
        let tile_position = world_to_grid_unbounded(world_position);
        min_x = min_x.min(tile_position.x);
        max_x = max_x.max(tile_position.x);
        min_y = min_y.min(tile_position.y);
        max_y = max_y.max(tile_position.y);
    }

    min_x = (min_x - TILE_VIEW_MARGIN).clamp(0, MAP_WIDTH - 1);
    max_x = (max_x + TILE_VIEW_MARGIN).clamp(0, MAP_WIDTH - 1);
    min_y = (min_y - TILE_VIEW_MARGIN).clamp(0, MAP_HEIGHT - 1);
    max_y = (max_y + TILE_VIEW_MARGIN).clamp(0, MAP_HEIGHT - 1);

    let mut positions = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            positions.push(GridPosition { x, y });
        }
    }

    Some(positions)
}
