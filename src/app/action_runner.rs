use bevy::{ecs::query::QueryFilter, prelude::*};

use super::components::{Building, MapPosition, Unit};
use super::interaction::{ActionLog, AvailableAction};
use super::resources::{AppMeshes, GameState, SelectedBuilding, SelectedUnit};
use super::selection::clear_selection;
use super::sync::{apply_building_events, apply_game_events, log_combat_events};
use crate::{Action, Event};

pub(super) fn run_available_action<F: QueryFilter, G: QueryFilter>(
    available_action: AvailableAction,
    commands: &mut Commands,
    app_meshes: &AppMeshes,
    materials: &mut Assets<ColorMaterial>,
    game: &mut GameState,
    selected_unit: &mut SelectedUnit,
    selected_building: &mut SelectedBuilding,
    units: &mut Query<(Entity, &mut MapPosition, &mut Transform, &Unit), F>,
    buildings: &Query<(Entity, &MapPosition, &Building), G>,
) {
    match available_action {
        AvailableAction::SelectUnit { entity, position } => {
            selected_unit.0 = Some(entity);
            selected_building.0 = None;
            info!("Unite selectionnee en ({}, {}).", position.x, position.y);
        }
        AvailableAction::SelectBuilding {
            entity,
            kind,
            position,
        } => {
            selected_unit.0 = None;
            selected_building.0 = Some(entity);
            info!(
                "{:?} selectionne en ({}, {}).",
                kind, position.x, position.y
            );
        }
        AvailableAction::ApplyGameAction {
            action,
            clear_selection_after,
            log,
        } => apply_game_action(
            action,
            clear_selection_after,
            log,
            commands,
            app_meshes,
            materials,
            game,
            selected_unit,
            selected_building,
            units,
            buildings,
        ),
        AvailableAction::None => {}
    }
}

fn apply_game_action<F: QueryFilter, G: QueryFilter>(
    action: Action,
    clear_selection_after: bool,
    log: ActionLog,
    commands: &mut Commands,
    app_meshes: &AppMeshes,
    materials: &mut Assets<ColorMaterial>,
    game: &mut GameState,
    selected_unit: &mut SelectedUnit,
    selected_building: &mut SelectedBuilding,
    units: &mut Query<(Entity, &mut MapPosition, &mut Transform, &Unit), F>,
    buildings: &Query<(Entity, &MapPosition, &Building), G>,
) {
    match game.0.apply(action) {
        Ok(events) => {
            apply_game_events(&events, commands, &app_meshes.unit, materials, units);
            apply_building_events(&events, commands, buildings);
            if clear_selection_after {
                clear_selection(selected_unit, selected_building);
            }
            log_action_success(log, &events);
        }
        Err(error) => log_action_error(log, error),
    }
}

fn log_action_success(log: ActionLog, events: &[Event]) {
    match log {
        ActionLog::MoveUnit { to } => {
            info!("Villageois deplace en ({}, {}).", to.x, to.y);
        }
        ActionLog::AttackBuilding {
            target_kind,
            target_position,
        } => {
            info!(
                "{:?} ennemi attaque en ({}, {}).",
                target_kind, target_position.x, target_position.y
            );
            log_combat_events(events);
        }
        ActionLog::AttackUnit | ActionLog::AttackWithBuilding => log_combat_events(events),
    }
}

fn log_action_error(log: ActionLog, error: crate::GameError) {
    match log {
        ActionLog::MoveUnit { .. } => info!("Deplacement refuse: {:?}.", error),
        ActionLog::AttackUnit => info!("Attaque refusee: {:?}.", error),
        ActionLog::AttackBuilding { .. } => info!("Attaque de batiment refusee: {:?}.", error),
        ActionLog::AttackWithBuilding => info!("Tir de tour refuse: {:?}.", error),
    }
}
