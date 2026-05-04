use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Building, MapPosition, Unit};
use super::grid::{cursor_grid_position, is_inside_map};
use super::interaction::{context_menu_lines, InteractionIntent};
use super::resources::{ContextMenu, FactionSelection, GameState, SelectedBuilding, SelectedUnit};
use crate::Camp;

pub(super) fn handle_context_menu_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    game: Res<GameState>,
    faction_selection: Res<FactionSelection>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
    units: Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    mut context_menu: ResMut<ContextMenu>,
) {
    if faction_selection.0 || !mouse_buttons.just_pressed(MouseButton::Right) {
        return;
    }

    context_menu.lines.clear();
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(screen_position) = window.cursor_position() else {
        return;
    };
    let Some(clicked_position) = cursor_grid_position(&windows, &cameras) else {
        return;
    };
    if !is_inside_map(clicked_position) {
        return;
    }

    context_menu.screen_position =
        Vec2::new(screen_position.x, window.height() - screen_position.y);
    context_menu.lines = context_menu_lines(
        InteractionIntent::ContextMenu {
            position: clicked_position,
        },
        game.0.faction(Camp::Human),
        &selected_unit,
        &selected_building,
        &units,
        &buildings,
    );
}
