use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{MapPosition, Unit};
use super::grid::{cursor_grid_position, is_inside_map};
use super::resources::{GameState, SelectedUnit};
use super::setup::spawn_building;
use super::sync::{apply_game_events, log_resource_events};
use crate::{Action, BuildingKind, Camp, Event};

pub(super) fn handle_human_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut game: ResMut<GameState>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit)>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(clicked_position) = cursor_grid_position(&windows, &cameras) else {
        return;
    };

    if !is_inside_map(clicked_position) {
        selected_unit.0 = None;
        return;
    }

    if let Some((entity, _, _, _)) = units
        .iter()
        .find(|(_, position, _, unit)| position.0 == clicked_position && unit.camp == Camp::Human)
    {
        selected_unit.0 = Some(entity);
        info!(
            "Villageois selectionne en ({}, {}).",
            clicked_position.x, clicked_position.y
        );
        return;
    }

    let Some(selected_entity) = selected_unit.0 else {
        return;
    };

    let unit_id = {
        let Ok((_, _, _, unit)) = units.get_mut(selected_entity) else {
            selected_unit.0 = None;
            return;
        };

        if unit.camp != Camp::Human {
            return;
        }

        unit.id
    };

    if let Some((_, _, _, target)) = units
        .iter()
        .find(|(_, position, _, unit)| position.0 == clicked_position && unit.camp != Camp::Human)
    {
        match game.0.apply(Action::AttackUnit {
            attacker_id: unit_id,
            target_id: target.id,
        }) {
            Ok(events) => {
                apply_game_events(&events, &mut commands, &mut units);
                selected_unit.0 = None;
                log_combat_events(&events);
            }
            Err(error) => {
                info!("Attaque refusee: {:?}.", error);
            }
        }
        return;
    }

    match game.0.apply(Action::MoveUnit {
        unit_id,
        to: clicked_position,
    }) {
        Ok(events) => {
            apply_game_events(&events, &mut commands, &mut units);
            selected_unit.0 = None;
            info!(
                "Villageois deplace en ({}, {}).",
                clicked_position.x, clicked_position.y
            );
        }
        Err(error) => {
            info!("Deplacement refuse: {:?}.", error);
        }
    }
}

pub(super) fn handle_build_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut game: ResMut<GameState>,
    selected_unit: Res<SelectedUnit>,
    units: Query<&Unit>,
) {
    let build_kind = if keyboard.just_pressed(KeyCode::KeyB) {
        BuildingKind::GoldMine
    } else if keyboard.just_pressed(KeyCode::KeyF) {
        BuildingKind::Farm
    } else {
        return;
    };

    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(selected_entity) = selected_unit.0 else {
        info!("Selectionne un villageois avant de construire un batiment.");
        return;
    };

    let Ok(unit) = units.get(selected_entity) else {
        return;
    };
    let unit_id = unit.id;
    let action = match build_kind {
        BuildingKind::GoldMine => Action::BuildGoldMine { unit_id },
        BuildingKind::Farm => Action::BuildFarm { unit_id },
    };

    match game.0.apply(action) {
        Ok(events) => {
            for event in &events {
                if let Event::BuildingConstructed {
                    camp,
                    kind,
                    position,
                } = *event
                {
                    spawn_building(&mut commands, camp, kind, position);
                    info!("{:?} construit en ({}, {}).", kind, position.x, position.y);
                }
            }
        }
        Err(error) => {
            info!("Construction refusee: {:?}.", error);
        }
    }
}

pub(super) fn handle_end_turn_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameState>,
    mut selected_unit: ResMut<SelectedUnit>,
) {
    if !keyboard.just_pressed(KeyCode::Space) && !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }

    if let Ok(events) = game.0.apply(Action::EndTurn) {
        selected_unit.0 = None;
        log_resource_events(&events);
        info!("Tour: IA.");
    }
}

pub(super) fn run_ai_turn(
    mut commands: Commands,
    mut game: ResMut<GameState>,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit)>,
) {
    if game.0.current_turn() != Camp::Ai {
        return;
    }

    let Ok(events) = game.0.apply(Action::RunAiTurn) else {
        return;
    };

    apply_game_events(&events, &mut commands, &mut units);
    for event in events {
        match event {
            Event::UnitMoved { unit_id, to, .. } => {
                if game.0.villager_id(Camp::Ai) == Some(unit_id) {
                    info!("L'IA deplace son villageois en ({}, {}).", to.x, to.y);
                }
            }
            Event::GoldProduced {
                camp,
                amount,
                total,
            } => {
                info!("{:?} produit {} or (total: {}).", camp, amount, total);
            }
            Event::FoodProduced {
                camp,
                amount,
                total,
            } => {
                info!(
                    "{:?} produit {} nourriture (total: {}).",
                    camp, amount, total
                );
            }
            _ => {}
        }
    }

    info!("Tour: Humain.");
}

fn log_combat_events(events: &[Event]) {
    for event in events {
        match *event {
            Event::UnitDamaged {
                unit_id,
                amount,
                remaining_health,
            } => {
                info!(
                    "Unite {:?} subit {} degats (PV restants: {}).",
                    unit_id, amount, remaining_health
                );
            }
            Event::UnitDefeated { unit_id } => {
                info!("Unite {:?} vaincue.", unit_id);
            }
            _ => {}
        }
    }
}
