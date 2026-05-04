use bevy::{prelude::*, window::PrimaryWindow};

use super::action_runner::run_available_action;
use super::components::{Building, MapPosition, Unit};
use super::constants::{MAP_HEIGHT, MAP_WIDTH};
use super::grid::{cursor_grid_position, is_inside_map};
use super::interaction::{available_action_for_intent, InteractionIntent};
use super::resources::{
    AppMeshes, ContextMenu, FactionSelection, GameState, SelectedBuilding, SelectedUnit,
};
use super::selection::clear_selection;
use super::setup::spawn_building;
use super::sync::{apply_game_events, log_resource_events};
use crate::{Action, BuildingKind, Camp, Event, Faction, Game, GridPosition};

pub(super) fn handle_faction_selection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameState>,
    mut faction_selection: ResMut<FactionSelection>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
) {
    if !faction_selection.0 {
        return;
    }

    let Some(human_faction) = selected_faction(&keyboard) else {
        return;
    };
    let ai_faction = default_ai_faction(human_faction);

    game.0 = Game::new_single_player(human_faction, ai_faction, MAP_WIDTH, MAP_HEIGHT);
    faction_selection.0 = false;
    clear_selection(&mut selected_unit, &mut selected_building);
    info!(
        "Civilisation choisie: {:?}. Adversaire: {:?}.",
        human_faction, ai_faction
    );
}

fn selected_faction(keyboard: &ButtonInput<KeyCode>) -> Option<Faction> {
    if keyboard.just_pressed(KeyCode::Digit1) {
        Some(Faction::Valdorian)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(Faction::Kharzun)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(Faction::Sylvans)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(Faction::Necrarchs)
    } else {
        None
    }
}

fn default_ai_faction(human_faction: Faction) -> Faction {
    if human_faction == Faction::Kharzun {
        Faction::Valdorian
    } else {
        Faction::Kharzun
    }
}

pub(super) fn handle_human_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    app_meshes: Res<AppMeshes>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut game: ResMut<GameState>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut context_menu: ResMut<ContextMenu>,
    faction_selection: Res<FactionSelection>,
    mut units: ParamSet<(
        Query<(Entity, &MapPosition, &Unit), Without<Building>>,
        Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    )>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) {
    if faction_selection.0 || !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    context_menu.lines.clear();

    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(clicked_position) = cursor_grid_position(&windows, &cameras) else {
        return;
    };

    if !is_inside_map(clicked_position) {
        clear_selection(&mut selected_unit, &mut selected_building);
        return;
    }

    let available_action = available_action_for_intent(
        InteractionIntent::PrimaryClick {
            position: clicked_position,
        },
        &selected_unit,
        &selected_building,
        &units.p0(),
        &buildings,
    );
    run_available_action(
        available_action,
        &mut commands,
        &app_meshes,
        &mut materials,
        &mut game,
        &mut selected_unit,
        &mut selected_building,
        &mut units.p1(),
        &buildings,
    );
}

pub(super) fn handle_build_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    app_meshes: Res<AppMeshes>,
    mut game: ResMut<GameState>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
    faction_selection: Res<FactionSelection>,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: Query<(Entity, &Building, &MapPosition), Without<Unit>>,
) {
    if faction_selection.0 {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        handle_recruit_input(
            &mut commands,
            &mut materials,
            &app_meshes.unit,
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
            &mut materials,
            &app_meshes.unit,
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

    if keyboard.just_pressed(KeyCode::KeyC) {
        handle_recruit_input(
            &mut commands,
            &mut materials,
            &app_meshes.unit,
            &mut game,
            &selected_building,
            &mut units,
            &buildings,
            BuildingKind::Barracks,
            |building_position| Action::RecruitUniqueUnit { building_position },
            "Selectionne une caserne avant de recruter une unite unique.",
            "Selectionne une caserne alliee pour recruter une unite unique.",
            "Unite unique",
        );
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyV) {
        handle_recruit_input(
            &mut commands,
            &mut materials,
            &app_meshes.unit,
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
        handle_research_input(
            &mut game,
            &selected_building,
            &buildings,
            Action::ResearchMilitaryTraining,
        );
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyH) {
        handle_research_input(
            &mut game,
            &selected_building,
            &buildings,
            Action::ResearchAgriculture,
        );
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
    } else if keyboard.just_pressed(KeyCode::KeyO) {
        BuildingKind::Watchtower
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
        BuildingKind::Watchtower => Action::BuildWatchtower { unit_id },
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
                    spawn_building(
                        &mut commands,
                        &app_meshes.building,
                        &mut materials,
                        camp,
                        kind,
                        position,
                    );
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
    materials: &mut Assets<ColorMaterial>,
    unit_mesh: &Handle<Mesh>,
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
            apply_game_events(&events, commands, unit_mesh, materials, units);
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
    action: Action,
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

    match game.0.apply(action) {
        Ok(events) => log_resource_events(&events),
        Err(error) => info!("Recherche refusee: {:?}.", error),
    }
}

pub(super) fn handle_end_turn_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameState>,
    faction_selection: Res<FactionSelection>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
) {
    if faction_selection.0 {
        return;
    }

    if !keyboard.just_pressed(KeyCode::Space) && !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }

    if let Ok(events) = game.0.apply(Action::EndTurn) {
        clear_selection(&mut selected_unit, &mut selected_building);
        log_resource_events(&events);
        info!("Tour: IA.");
    }
}

pub(super) fn run_ai_turn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    app_meshes: Res<AppMeshes>,
    mut game: ResMut<GameState>,
    faction_selection: Res<FactionSelection>,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit)>,
) {
    if faction_selection.0 {
        return;
    }

    if game.0.current_turn() != Camp::Ai {
        return;
    }

    let Ok(events) = game.0.apply(Action::RunAiTurn) else {
        return;
    };

    apply_game_events(
        &events,
        &mut commands,
        &app_meshes.unit,
        &mut materials,
        &mut units,
    );
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
