use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Building, MapPosition, Unit};
use super::grid::{cursor_grid_position, is_inside_map};
use super::resources::{GameState, SelectedBuilding, SelectedUnit};
use super::setup::spawn_building;
use super::sync::{apply_building_events, apply_game_events, log_resource_events};
use crate::{Action, BuildingKind, Camp, Event, GridPosition};

pub(super) fn handle_human_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut game: ResMut<GameState>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
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
        selected_building.0 = None;
        return;
    }

    if let Some((entity, _, _, _)) = units
        .iter()
        .find(|(_, position, _, unit)| position.0 == clicked_position && unit.camp == Camp::Human)
    {
        selected_unit.0 = Some(entity);
        selected_building.0 = None;
        info!(
            "Unite selectionnee en ({}, {}).",
            clicked_position.x, clicked_position.y
        );
        return;
    }

    if selected_unit.0.is_none() {
        if let Some((entity, _, building)) = buildings.iter().find(|(_, position, building)| {
            position.0 == clicked_position
                && building.camp == Camp::Human
                && matches!(
                    building.kind,
                    BuildingKind::Barracks
                        | BuildingKind::Forum
                        | BuildingKind::Market
                        | BuildingKind::University
                )
        }) {
            selected_building.0 = Some(entity);
            info!(
                "{:?} selectionne en ({}, {}).",
                building.kind, clicked_position.x, clicked_position.y
            );
            return;
        }
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
                selected_building.0 = None;
                log_combat_events(&events);
            }
            Err(error) => {
                info!("Attaque refusee: {:?}.", error);
            }
        }
        return;
    }

    if let Some((_, _, target)) = buildings.iter().find(|(_, position, building)| {
        position.0 == clicked_position && building.camp != Camp::Human
    }) {
        match game.0.apply(Action::AttackBuilding {
            attacker_id: unit_id,
            target_position: clicked_position,
        }) {
            Ok(events) => {
                apply_game_events(&events, &mut commands, &mut units);
                apply_building_events(&events, &mut commands, &buildings);
                selected_unit.0 = None;
                selected_building.0 = None;
                info!(
                    "{:?} ennemi attaque en ({}, {}).",
                    target.kind, clicked_position.x, clicked_position.y
                );
                log_combat_events(&events);
            }
            Err(error) => {
                info!("Attaque de batiment refusee: {:?}.", error);
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
    selected_building: Res<SelectedBuilding>,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: Query<(Entity, &Building, &MapPosition), Without<Unit>>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        handle_recruit_input(
            &mut commands,
            &mut game,
            &selected_building,
            &mut units,
            &buildings,
            BuildingKind::Barracks,
            |building_position| Action::RecruitSoldier { building_position },
            "Selectionne une caserne avant de recruter un soldat.",
            "Selectionne une caserne alliee pour recruter un soldat.",
            "Soldat",
        );
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyA) {
        handle_recruit_input(
            &mut commands,
            &mut game,
            &selected_building,
            &mut units,
            &buildings,
            BuildingKind::Barracks,
            |building_position| Action::RecruitArcher { building_position },
            "Selectionne une caserne avant de recruter un archer.",
            "Selectionne une caserne alliee pour recruter un archer.",
            "Archer",
        );
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyV) {
        handle_recruit_input(
            &mut commands,
            &mut game,
            &selected_building,
            &mut units,
            &buildings,
            BuildingKind::Forum,
            |building_position| Action::RecruitVillager { building_position },
            "Selectionne un forum avant de recruter un villageois.",
            "Selectionne un forum allie pour recruter un villageois.",
            "Villageois",
        );
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyG) {
        handle_trade_input(
            &mut game,
            &selected_building,
            &buildings,
            Action::TradeGoldForFood { amount: 10 },
        );
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyN) {
        handle_trade_input(
            &mut game,
            &selected_building,
            &buildings,
            Action::TradeFoodForGold { amount: 10 },
        );
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyY) {
        handle_research_input(&mut game, &selected_building, &buildings);
        return;
    }

    let build_kind = if keyboard.just_pressed(KeyCode::KeyB) {
        BuildingKind::GoldMine
    } else if keyboard.just_pressed(KeyCode::KeyF) {
        BuildingKind::Farm
    } else if keyboard.just_pressed(KeyCode::KeyT) {
        BuildingKind::Forum
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        BuildingKind::Barracks
    } else if keyboard.just_pressed(KeyCode::KeyM) {
        BuildingKind::Market
    } else if keyboard.just_pressed(KeyCode::KeyU) {
        BuildingKind::University
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

    let Ok((_, _, _, unit)) = units.get(selected_entity) else {
        return;
    };
    let unit_id = unit.id;
    let action = match build_kind {
        BuildingKind::GoldMine => Action::BuildGoldMine { unit_id },
        BuildingKind::Farm => Action::BuildFarm { unit_id },
        BuildingKind::Forum => Action::BuildForum { unit_id },
        BuildingKind::Barracks => Action::BuildBarracks { unit_id },
        BuildingKind::Market => Action::BuildMarket { unit_id },
        BuildingKind::University => Action::BuildUniversity { unit_id },
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

fn handle_recruit_input(
    commands: &mut Commands,
    game: &mut ResMut<GameState>,
    selected_building: &Res<SelectedBuilding>,
    units: &mut Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &Building, &MapPosition), Without<Unit>>,
    expected_kind: BuildingKind,
    action: impl FnOnce(GridPosition) -> Action,
    missing_selection_message: &str,
    wrong_building_message: &str,
    recruited_label: &str,
) {
    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(selected_entity) = selected_building.0 else {
        info!("{}", missing_selection_message);
        return;
    };

    let Ok((_, building, position)) = buildings.get(selected_entity) else {
        return;
    };

    if building.camp != Camp::Human || building.kind != expected_kind {
        info!("{}", wrong_building_message);
        return;
    }

    let building_position = position.0;

    match game.0.apply(action(building_position)) {
        Ok(events) => {
            apply_game_events(&events, commands, units);
            for event in events {
                if let Event::UnitRecruited {
                    unit_id, position, ..
                } = event
                {
                    info!(
                        "{} {:?} recrute en ({}, {}).",
                        recruited_label, unit_id, position.x, position.y
                    );
                }
            }
        }
        Err(error) => {
            info!("Recrutement refuse: {:?}.", error);
        }
    }
}

fn handle_trade_input(
    game: &mut ResMut<GameState>,
    selected_building: &Res<SelectedBuilding>,
    buildings: &Query<(Entity, &Building, &MapPosition), Without<Unit>>,
    action: Action,
) {
    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(selected_entity) = selected_building.0 else {
        info!("Selectionne un marche avant d'echanger des ressources.");
        return;
    };

    let Ok((_, building, _)) = buildings.get(selected_entity) else {
        return;
    };

    if building.camp != Camp::Human || building.kind != BuildingKind::Market {
        info!("Selectionne un marche allie pour echanger des ressources.");
        return;
    }

    match game.0.apply(action) {
        Ok(events) => log_resource_events(&events),
        Err(error) => info!("Echange refuse: {:?}.", error),
    }
}

fn handle_research_input(
    game: &mut ResMut<GameState>,
    selected_building: &Res<SelectedBuilding>,
    buildings: &Query<(Entity, &Building, &MapPosition), Without<Unit>>,
) {
    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(selected_entity) = selected_building.0 else {
        info!("Selectionne une universite avant de rechercher une technologie.");
        return;
    };

    let Ok((_, building, _)) = buildings.get(selected_entity) else {
        return;
    };

    if building.camp != Camp::Human || building.kind != BuildingKind::University {
        info!("Selectionne une universite alliee pour rechercher une technologie.");
        return;
    }

    match game.0.apply(Action::ResearchMilitaryTraining) {
        Ok(events) => log_resource_events(&events),
        Err(error) => info!("Recherche refusee: {:?}.", error),
    }
}

pub(super) fn handle_end_turn_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameState>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
) {
    if !keyboard.just_pressed(KeyCode::Space) && !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }

    if let Ok(events) = game.0.apply(Action::EndTurn) {
        selected_unit.0 = None;
        selected_building.0 = None;
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
            Event::TechnologyProduced {
                camp,
                amount,
                total,
            } => {
                info!(
                    "{:?} produit {} technologie (total: {}).",
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
            Event::BuildingDamaged {
                kind,
                position,
                amount,
                remaining_health,
                ..
            } => {
                info!(
                    "{:?} en ({}, {}) subit {} degats (PV restants: {}).",
                    kind, position.x, position.y, amount, remaining_health
                );
            }
            Event::BuildingDestroyed { kind, position, .. } => {
                info!("{:?} detruit en ({}, {}).", kind, position.x, position.y);
            }
            Event::GameWon { camp } => {
                info!("Victoire {:?}.", camp);
            }
            _ => {}
        }
    }
}
