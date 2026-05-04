use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Building, MapPosition, Unit};
use super::constants::{MAP_HEIGHT, MAP_WIDTH};
use super::grid::{cursor_grid_position, is_inside_map};
use super::resources::{
    AppMeshes, ContextMenu, FactionSelection, GameState, SelectedBuilding, SelectedUnit,
};
use super::setup::spawn_building;
use super::sync::{apply_building_events, apply_game_events, log_resource_events};
use crate::{Action, BuildingKind, Camp, Event, Faction, Game, GridPosition, UnitKind};

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

pub(super) fn handle_deselect_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    faction_selection: Res<FactionSelection>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut context_menu: ResMut<ContextMenu>,
) {
    if faction_selection.0 {
        return;
    }

    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    context_menu.lines.clear();
    if selected_unit.0.is_some() || selected_building.0.is_some() {
        clear_selection(&mut selected_unit, &mut selected_building);
        info!("Selection annulee.");
    }
}

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
        &game,
        &selected_unit,
        &selected_building,
        &units,
        &buildings,
        clicked_position,
    );
}

fn context_menu_lines(
    game: &GameState,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    clicked_position: GridPosition,
) -> Vec<String> {
    let mut lines = vec![format!(
        "Case ({}, {})",
        clicked_position.x, clicked_position.y
    )];

    if let Some((_, _, unit)) = units
        .iter()
        .find(|(_, position, _)| position.0 == clicked_position)
    {
        lines.push(format!("Unite: {:?} {:?}", unit.camp, unit.kind));
        if unit.camp == Camp::Human {
            lines.push("Selectionner: clic gauche".to_string());
            lines.push(unit_context_actions(unit.kind).to_string());
        } else if selected_unit.0.is_some() {
            lines.push("Attaquer: clic gauche".to_string());
        } else if selected_watchtower(selected_building, buildings) {
            lines.push("Tir tour: clic gauche".to_string());
        } else {
            lines.push("Selectionne unite ou tour pour attaquer.".to_string());
        }
        return lines;
    }

    if let Some((_, _, building)) = buildings
        .iter()
        .find(|(_, position, _)| position.0 == clicked_position)
    {
        lines.push(format!("Batiment: {:?} {:?}", building.camp, building.kind));
        if building.camp == Camp::Human {
            lines.push("Selectionner: clic gauche".to_string());
            lines.push(building_context_actions(
                building.kind,
                game.0.faction(building.camp),
            ));
        } else if selected_unit.0.is_some() {
            lines.push("Attaquer: clic gauche".to_string());
        } else {
            lines.push("Selectionne unite pour attaquer.".to_string());
        }
        return lines;
    }

    if let Some(selected_entity) = selected_unit.0 {
        if let Ok((_, _, unit)) = units.get(selected_entity) {
            if unit.camp == Camp::Human {
                lines.push("Deplacer: clic gauche".to_string());
                if unit.kind == UnitKind::Villager {
                    lines.push("Construire: B/F/T/R/M/U/O".to_string());
                }
            }
        }
    } else {
        lines.push("Selectionne unite ou batiment allie.".to_string());
    }

    lines
}

fn selected_watchtower(
    selected_building: &SelectedBuilding,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) -> bool {
    let Some(selected_entity) = selected_building.0 else {
        return false;
    };
    let Ok((_, _, building)) = buildings.get(selected_entity) else {
        return false;
    };

    building.camp == Camp::Human && building.kind == BuildingKind::Watchtower
}

fn unit_context_actions(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Villager => "Actions: deplacer, construire, attaquer",
        UnitKind::Soldier
        | UnitKind::Archer
        | UnitKind::ValdorianLegionary
        | UnitKind::KharzunShieldbreaker
        | UnitKind::ElyrPathfinder
        | UnitKind::ObsidianBoneServant => "Actions: deplacer, attaquer",
    }
}

fn building_context_actions(kind: BuildingKind, faction: Faction) -> String {
    match kind {
        BuildingKind::Forum => "Actions: V villageois".to_string(),
        BuildingKind::Barracks => format!(
            "Actions: S soldat | A archer | C {}",
            unique_unit_label(faction)
        ),
        BuildingKind::Market => "Actions: G or->nourriture | N nourriture->or".to_string(),
        BuildingKind::University => "Actions: H agriculture | Y militaire".to_string(),
        BuildingKind::Watchtower => "Actions: clic gauche unite ennemie = tir".to_string(),
        BuildingKind::GoldMine | BuildingKind::Farm => "Actions: production passive".to_string(),
    }
}

fn unique_unit_label(faction: Faction) -> &'static str {
    match faction {
        Faction::Valdorian => "Legionnaire",
        Faction::Kharzun => "Brise-bouclier",
        Faction::Sylvans => "Pisteur",
        Faction::Necrarchs => "Serviteur osseux",
    }
}

fn clear_selection(selected_unit: &mut SelectedUnit, selected_building: &mut SelectedBuilding) {
    selected_unit.0 = None;
    selected_building.0 = None;
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
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) {
    if faction_selection.0 {
        return;
    }

    if !mouse_buttons.just_pressed(MouseButton::Left) {
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
                        | BuildingKind::Watchtower
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

    if let Some(selected_entity) = selected_building.0 {
        if let Some((_, _, _, target)) = units.iter().find(|(_, position, _, unit)| {
            position.0 == clicked_position && unit.camp != Camp::Human
        }) {
            let Ok((_, building_position, building)) = buildings.get(selected_entity) else {
                selected_building.0 = None;
                return;
            };

            if building.kind != BuildingKind::Watchtower || building.camp != Camp::Human {
                return;
            }

            match game.0.apply(Action::AttackWithBuilding {
                building_position: building_position.0,
                target_id: target.id,
            }) {
                Ok(events) => {
                    apply_game_events(
                        &events,
                        &mut commands,
                        &app_meshes.unit,
                        &mut materials,
                        &mut units,
                    );
                    clear_selection(&mut selected_unit, &mut selected_building);
                    log_combat_events(&events);
                }
                Err(error) => {
                    info!("Tir de tour refuse: {:?}.", error);
                }
            }
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
                apply_game_events(
                    &events,
                    &mut commands,
                    &app_meshes.unit,
                    &mut materials,
                    &mut units,
                );
                clear_selection(&mut selected_unit, &mut selected_building);
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
                apply_game_events(
                    &events,
                    &mut commands,
                    &app_meshes.unit,
                    &mut materials,
                    &mut units,
                );
                apply_building_events(&events, &mut commands, &buildings);
                clear_selection(&mut selected_unit, &mut selected_building);
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
            apply_game_events(
                &events,
                &mut commands,
                &app_meshes.unit,
                &mut materials,
                &mut units,
            );
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
            Event::BuildingActed { kind, position, .. } => {
                info!("{:?} agit en ({}, {}).", kind, position.x, position.y);
            }
            Event::GameWon { camp } => {
                info!("Victoire {:?}.", camp);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_selection_removes_selected_unit_and_building() {
        let mut selected_unit = SelectedUnit(Some(Entity::from_raw(1)));
        let mut selected_building = SelectedBuilding(Some(Entity::from_raw(2)));

        clear_selection(&mut selected_unit, &mut selected_building);

        assert_eq!(selected_unit.0, None);
        assert_eq!(selected_building.0, None);
    }
}
