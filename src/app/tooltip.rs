use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Building, HoverInfoPanelText, MapPosition, TooltipText, Unit};
use super::grid::{cursor_grid_position, is_inside_map};
use super::interaction::{context_menu_lines, InteractionIntent};
use super::resources::{ContextMenu, FactionSelection, GameState, SelectedBuilding, SelectedUnit};
use crate::{Camp, GridPosition};

pub(super) fn spawn_tooltip(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.94, 0.96, 0.88),
                ..default()
            },
        )
        .with_style(Style {
            display: Display::None,
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            padding: UiRect::all(Val::Px(8.0)),
            max_width: Val::Px(320.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .with_background_color(Color::srgba(0.02, 0.024, 0.032, 0.9)),
        TooltipText,
    ));
}

pub(super) fn spawn_hover_info_panel(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 15.0,
                color: Color::srgb(0.92, 0.94, 0.86),
                ..default()
            },
        )
        .with_style(Style {
            display: Display::None,
            position_type: PositionType::Absolute,
            right: Val::Px(12.0),
            top: Val::Px(72.0),
            width: Val::Px(304.0),
            padding: UiRect::all(Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .with_background_color(Color::srgba(0.03, 0.035, 0.045, 0.88)),
        HoverInfoPanelText,
    ));
}

pub(super) fn update_hover_tooltip(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    game: Res<GameState>,
    faction_selection: Res<FactionSelection>,
    context_menu: Res<ContextMenu>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
    units: Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    button_interactions: Query<&Interaction, With<Button>>,
    mut tooltips: Query<(&mut Text, &mut Style), With<TooltipText>>,
) {
    let Ok((mut text, mut style)) = tooltips.get_single_mut() else {
        return;
    };

    if faction_selection.0
        || !context_menu.lines.is_empty()
        || button_interactions
            .iter()
            .any(|interaction| *interaction != Interaction::None)
    {
        hide_tooltip(&mut style);
        return;
    }

    let Ok(window) = windows.get_single() else {
        hide_tooltip(&mut style);
        return;
    };
    let Some(screen_position) = window.cursor_position() else {
        hide_tooltip(&mut style);
        return;
    };
    let Some(position) = cursor_grid_position(&windows, &cameras) else {
        hide_tooltip(&mut style);
        return;
    };
    if !is_inside_map(position) {
        hide_tooltip(&mut style);
        return;
    }
    if entity_hovered(&units, &buildings, position) {
        hide_tooltip(&mut style);
        return;
    }

    text.sections[0].value = tooltip_text(
        &game,
        &selected_unit,
        &selected_building,
        &units,
        &buildings,
        position,
    );
    style.display = Display::Flex;
    let left = (screen_position.x + 16.0)
        .min(window.width() - 340.0)
        .max(12.0);
    let top = (window.height() - screen_position.y + 16.0)
        .min(window.height() - 220.0)
        .max(12.0);
    style.left = Val::Px(left);
    style.top = Val::Px(top);
}

pub(super) fn update_hover_info_panel(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    game: Res<GameState>,
    faction_selection: Res<FactionSelection>,
    context_menu: Res<ContextMenu>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
    units: Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    button_interactions: Query<&Interaction, With<Button>>,
    mut panels: Query<(&mut Text, &mut Style), With<HoverInfoPanelText>>,
) {
    let Ok((mut text, mut style)) = panels.get_single_mut() else {
        return;
    };

    if faction_selection.0
        || !context_menu.lines.is_empty()
        || button_interactions
            .iter()
            .any(|interaction| *interaction != Interaction::None)
    {
        hide_tooltip(&mut style);
        return;
    }

    let Some(position) = cursor_grid_position(&windows, &cameras) else {
        hide_tooltip(&mut style);
        return;
    };
    if !is_inside_map(position) {
        hide_tooltip(&mut style);
        return;
    }

    if let Some((_, _, unit)) = units
        .iter()
        .find(|(_, unit_position, _)| unit_position.0 == position)
    {
        text.sections[0].value = unit_panel_text(
            &game,
            &selected_unit,
            &selected_building,
            &units,
            &buildings,
            position,
            unit,
        );
        style.display = Display::Flex;
        return;
    }

    if let Some((_, _, building)) = buildings
        .iter()
        .find(|(_, building_position, _)| building_position.0 == position)
    {
        text.sections[0].value = building_panel_text(
            &game,
            &selected_unit,
            &selected_building,
            &units,
            &buildings,
            position,
            building,
        );
        style.display = Display::Flex;
        return;
    }

    hide_tooltip(&mut style);
}

fn hide_tooltip(style: &mut Style) {
    style.display = Display::None;
}

fn entity_hovered(
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    position: GridPosition,
) -> bool {
    units
        .iter()
        .any(|(_, unit_position, _)| unit_position.0 == position)
        || buildings
            .iter()
            .any(|(_, building_position, _)| building_position.0 == position)
}

fn unit_panel_text(
    game: &GameState,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    position: GridPosition,
    unit: &Unit,
) -> String {
    let health = game.0.unit_health(unit.id).unwrap_or(0);
    let moved = game.0.unit_has_moved(unit.id).unwrap_or(false);
    let acted = game.0.unit_has_acted(unit.id).unwrap_or(false);
    let mut lines = vec![
        "Survol unite".to_string(),
        format!("{:?} {:?}", unit.camp, unit.kind),
        format!("Position: {}, {}", position.x, position.y),
        format!("PV: {}", health),
        format!("Etat: bouge {} | agi {}", moved, acted),
        format!("Visibilite: {}", visibility_label(game, position)),
    ];
    lines.extend(hover_action_lines(
        game,
        selected_unit,
        selected_building,
        units,
        buildings,
        position,
    ));

    lines.join("\n")
}

fn building_panel_text(
    game: &GameState,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    position: GridPosition,
    building: &Building,
) -> String {
    let mut lines = vec![
        "Survol batiment".to_string(),
        format!("{:?} {:?}", building.camp, building.kind),
        format!("Position: {}, {}", position.x, position.y),
        format!("Visibilite: {}", visibility_label(game, position)),
    ];
    if let Some(resource) = game.0.natural_resource_at(position) {
        lines.push(format!("Ressource: {:?}", resource));
    }
    lines.extend(hover_action_lines(
        game,
        selected_unit,
        selected_building,
        units,
        buildings,
        position,
    ));

    lines.join("\n")
}

fn hover_action_lines(
    game: &GameState,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    position: GridPosition,
) -> Vec<String> {
    let mut lines = context_menu_lines(
        InteractionIntent::ContextMenu { position },
        game.0.faction(Camp::Human),
        selected_unit,
        selected_building,
        units,
        buildings,
    );
    if !lines.is_empty() {
        lines[0] = "Action".to_string();
    }
    lines
}

fn tooltip_text(
    game: &GameState,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    position: GridPosition,
) -> String {
    let mut lines = vec![format!("Case ({}, {})", position.x, position.y)];
    lines.push(format!("Visibilite: {}", visibility_label(game, position)));
    if let Some(resource) = game.0.natural_resource_at(position) {
        lines.push(format!("Ressource: {:?}", resource));
    }
    if let Some((_, _, unit)) = units
        .iter()
        .find(|(_, unit_position, _)| unit_position.0 == position)
    {
        let health = game.0.unit_health(unit.id).unwrap_or(0);
        let moved = game.0.unit_has_moved(unit.id).unwrap_or(false);
        let acted = game.0.unit_has_acted(unit.id).unwrap_or(false);
        lines.push(format!(
            "Unite: {:?} {:?} | PV {}",
            unit.camp, unit.kind, health
        ));
        lines.push(format!("Etat: bouge {} | agi {}", moved, acted));
    }
    if let Some((_, _, building)) = buildings
        .iter()
        .find(|(_, building_position, _)| building_position.0 == position)
    {
        lines.push(format!("Batiment: {:?} {:?}", building.camp, building.kind));
    }

    let action_lines = context_menu_lines(
        InteractionIntent::ContextMenu { position },
        game.0.faction(Camp::Human),
        selected_unit,
        selected_building,
        units,
        buildings,
    );
    lines.extend(action_lines.into_iter().skip(1));

    lines.join("\n")
}

fn visibility_label(game: &GameState, position: GridPosition) -> &'static str {
    if game.0.is_visible(Camp::Human, position) {
        "visible"
    } else if game.0.is_explored(Camp::Human, position) {
        "exploree"
    } else {
        "inconnue"
    }
}
