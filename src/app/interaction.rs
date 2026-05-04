use bevy::prelude::*;

use super::components::{Building, MapPosition, Unit};
use super::resources::{SelectedBuilding, SelectedUnit};
use crate::{Action, BuildingKind, Camp, Faction, GridPosition, UnitKind};

pub(super) enum InteractionIntent {
    PrimaryClick { position: GridPosition },
    ContextMenu { position: GridPosition },
}

#[derive(Clone, Copy)]
pub(super) enum AvailableAction {
    SelectUnit {
        entity: Entity,
        position: GridPosition,
    },
    SelectBuilding {
        entity: Entity,
        kind: BuildingKind,
        position: GridPosition,
    },
    ApplyGameAction {
        action: Action,
        clear_selection_after: bool,
        log: ActionLog,
    },
    None,
}

#[derive(Clone, Copy)]
pub(super) enum ActionLog {
    MoveUnit {
        to: GridPosition,
    },
    AttackUnit,
    AttackBuilding {
        target_kind: BuildingKind,
        target_position: GridPosition,
    },
    AttackWithBuilding,
}

pub(super) fn available_action_for_intent(
    intent: InteractionIntent,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) -> AvailableAction {
    match intent {
        InteractionIntent::PrimaryClick { position } => {
            primary_click_action(position, selected_unit, selected_building, units, buildings)
        }
        InteractionIntent::ContextMenu { .. } => AvailableAction::None,
    }
}

pub(super) fn context_menu_lines(
    intent: InteractionIntent,
    faction: Faction,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) -> Vec<String> {
    let InteractionIntent::ContextMenu { position } = intent else {
        return Vec::new();
    };

    let mut lines = vec![format!("Case ({}, {})", position.x, position.y)];

    if let Some((_, _, unit)) = units
        .iter()
        .find(|(_, unit_position, _)| unit_position.0 == position)
    {
        lines.push(format!("Unite: {:?} {:?}", unit.camp, unit.kind));
        let action =
            primary_click_action(position, selected_unit, selected_building, units, buildings);
        lines.extend(action_lines(action, unit.kind, None, faction));
        return lines;
    }

    if let Some((_, _, building)) = buildings
        .iter()
        .find(|(_, building_position, _)| building_position.0 == position)
    {
        lines.push(format!("Batiment: {:?} {:?}", building.camp, building.kind));
        let action =
            primary_click_action(position, selected_unit, selected_building, units, buildings);
        lines.extend(action_lines(
            action,
            UnitKind::Villager,
            Some(building.kind),
            faction,
        ));
        return lines;
    }

    let action = primary_click_action(position, selected_unit, selected_building, units, buildings);
    let unit_kind = selected_unit_kind(selected_unit, units).unwrap_or(UnitKind::Soldier);
    lines.extend(action_lines(action, unit_kind, None, faction));
    lines
}

fn primary_click_action(
    position: GridPosition,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) -> AvailableAction {
    if let Some((entity, _, _)) = units
        .iter()
        .find(|(_, unit_position, unit)| unit_position.0 == position && unit.camp == Camp::Human)
    {
        return AvailableAction::SelectUnit { entity, position };
    }

    if selected_unit.0.is_none() {
        if let Some((entity, _, building)) =
            buildings.iter().find(|(_, building_position, building)| {
                building_position.0 == position
                    && building.camp == Camp::Human
                    && is_selectable_building(building.kind)
            })
        {
            return AvailableAction::SelectBuilding {
                entity,
                kind: building.kind,
                position,
            };
        }
    }

    if let Some(action) = selected_building_action(position, selected_building, units, buildings) {
        return action;
    }

    let Some(selected_entity) = selected_unit.0 else {
        return AvailableAction::None;
    };
    let Ok((_, _, selected)) = units.get(selected_entity) else {
        return AvailableAction::None;
    };
    if selected.camp != Camp::Human {
        return AvailableAction::None;
    }

    if let Some((_, _, target)) = units
        .iter()
        .find(|(_, unit_position, unit)| unit_position.0 == position && unit.camp != Camp::Human)
    {
        return AvailableAction::ApplyGameAction {
            action: Action::AttackUnit {
                attacker_id: selected.id,
                target_id: target.id,
            },
            clear_selection_after: true,
            log: ActionLog::AttackUnit,
        };
    }

    if let Some((_, _, target)) = buildings.iter().find(|(_, building_position, building)| {
        building_position.0 == position && building.camp != Camp::Human
    }) {
        return AvailableAction::ApplyGameAction {
            action: Action::AttackBuilding {
                attacker_id: selected.id,
                target_position: position,
            },
            clear_selection_after: true,
            log: ActionLog::AttackBuilding {
                target_kind: target.kind,
                target_position: position,
            },
        };
    }

    AvailableAction::ApplyGameAction {
        action: Action::MoveUnit {
            unit_id: selected.id,
            to: position,
        },
        clear_selection_after: false,
        log: ActionLog::MoveUnit { to: position },
    }
}

fn selected_building_action(
    position: GridPosition,
    selected_building: &SelectedBuilding,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) -> Option<AvailableAction> {
    let selected_entity = selected_building.0?;
    let Some((_, _, target)) = units
        .iter()
        .find(|(_, unit_position, unit)| unit_position.0 == position && unit.camp != Camp::Human)
    else {
        return None;
    };
    let Ok((_, building_position, building)) = buildings.get(selected_entity) else {
        return None;
    };
    if building.kind != BuildingKind::Watchtower || building.camp != Camp::Human {
        return None;
    }

    Some(AvailableAction::ApplyGameAction {
        action: Action::AttackWithBuilding {
            building_position: building_position.0,
            target_id: target.id,
        },
        clear_selection_after: true,
        log: ActionLog::AttackWithBuilding,
    })
}

fn action_lines(
    action: AvailableAction,
    unit_kind: UnitKind,
    building_kind: Option<BuildingKind>,
    faction: Faction,
) -> Vec<String> {
    match action {
        AvailableAction::SelectUnit { .. } => vec![
            "Selectionner: clic gauche".to_string(),
            unit_context_actions(unit_kind).to_string(),
        ],
        AvailableAction::SelectBuilding { kind, .. } => vec![
            "Selectionner: clic gauche".to_string(),
            building_context_actions(kind, faction),
        ],
        AvailableAction::ApplyGameAction { log, .. } => match log {
            ActionLog::MoveUnit { .. } => move_lines(unit_kind),
            ActionLog::AttackUnit => vec!["Attaquer: clic gauche".to_string()],
            ActionLog::AttackBuilding { .. } => vec!["Attaquer: clic gauche".to_string()],
            ActionLog::AttackWithBuilding => vec!["Tir tour: clic gauche".to_string()],
        },
        AvailableAction::None => match building_kind {
            Some(_) => vec!["Selectionne unite ou batiment allie.".to_string()],
            None => vec!["Selectionne unite ou batiment allie.".to_string()],
        },
    }
}

fn selected_unit_kind(
    selected_unit: &SelectedUnit,
    units: &Query<(Entity, &MapPosition, &Unit), Without<Building>>,
) -> Option<UnitKind> {
    let selected_entity = selected_unit.0?;
    let Ok((_, _, unit)) = units.get(selected_entity) else {
        return None;
    };

    Some(unit.kind)
}

fn move_lines(unit_kind: UnitKind) -> Vec<String> {
    let mut lines = vec!["Deplacer: clic gauche".to_string()];
    if unit_kind == UnitKind::Villager {
        lines.push("Construire: B/F/T/R/M/U/O".to_string());
    }
    lines
}

fn is_selectable_building(kind: BuildingKind) -> bool {
    matches!(
        kind,
        BuildingKind::Barracks
            | BuildingKind::Forum
            | BuildingKind::Market
            | BuildingKind::University
            | BuildingKind::Watchtower
    )
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
        BuildingKind::Barracks => {
            format!(
                "Actions: S soldat | A archer | C {}",
                unique_unit_label(faction)
            )
        }
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
