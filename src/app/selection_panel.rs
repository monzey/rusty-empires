use bevy::prelude::*;

use super::components::{Building, MapPosition, SelectionPanelText, Unit};
use super::resources::{FactionSelection, GameState, SelectedBuilding, SelectedUnit};
use crate::{BuildingKind, Faction, UnitKind};

pub(super) fn update_selection_panel(
    game: Res<GameState>,
    faction_selection: Res<FactionSelection>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
    units: Query<(&Unit, &MapPosition)>,
    buildings: Query<(&Building, &MapPosition)>,
    mut panel: Query<&mut Text, With<SelectionPanelText>>,
) {
    let Ok(mut text) = panel.get_single_mut() else {
        return;
    };

    if faction_selection.0 {
        text.sections[0].value =
            "Selection: aucune\nChoisis une civilisation pour commencer.".to_string();
        return;
    }

    text.sections[0].value = selection_text(
        &game,
        &selected_unit,
        &selected_building,
        &units,
        &buildings,
    );
}

fn selection_text(
    game: &GameState,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(&Unit, &MapPosition)>,
    buildings: &Query<(&Building, &MapPosition)>,
) -> String {
    if let Some(entity) = selected_unit.0 {
        if let Ok((unit, position)) = units.get(entity) {
            return format!(
                "Selection: {:?} {:?} ({}, {})\nPV: {} | Bouge: {} | Agi: {}{}",
                unit.camp,
                unit.kind,
                position.0.x,
                position.0.y,
                game.0.unit_health(unit.id).unwrap_or(0),
                game.0.unit_has_moved(unit.id).unwrap_or(false),
                game.0.unit_has_acted(unit.id).unwrap_or(false),
                unit_actions(unit.kind),
            );
        }
    }

    if let Some(entity) = selected_building.0 {
        if let Ok((building, position)) = buildings.get(entity) {
            return format!(
                "Selection: {:?} {:?} ({}, {}){}",
                building.camp,
                building.kind,
                position.0.x,
                position.0.y,
                building_actions(building.kind, game.0.faction(building.camp)),
            );
        }
    }

    "Selection: aucune\nActions: selectionne une unite ou un batiment allie.".to_string()
}

fn unit_actions(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Villager => "\nActions: clic case = bouger | clic ennemi = attaquer\nConstruire: B mine 60o | F ferme 40o | T forum 250o/100n | R caserne 180o/80n | M marche 160o/80n | U universite 220o/120n | O tour 140o/40n",
        UnitKind::Soldier
        | UnitKind::Archer
        | UnitKind::ValdorianLegionary
        | UnitKind::KharzunShieldbreaker
        | UnitKind::ElyrPathfinder
        | UnitKind::ObsidianBoneServant => {
            "\nActions: clic case = bouger | clic ennemi = attaquer"
        }
    }
}

fn building_actions(kind: BuildingKind, faction: Faction) -> String {
    match kind {
        BuildingKind::Forum => "\nActions: V villageois 10n".to_string(),
        BuildingKind::Barracks => format!(
            "\nActions: S soldat 10n | A archer 10n/5o | C {} {}",
            unique_unit_label(faction),
            unique_unit_cost(faction),
        ),
        BuildingKind::Market => "\nActions: G 10o -> 5n | N 10n -> 5o".to_string(),
        BuildingKind::University => {
            "\nActions: H Agriculture 80t/60n | Y Entrainement militaire 10t".to_string()
        }
        BuildingKind::Watchtower => "\nActions: clic unite ennemie = tir portee 2-5".to_string(),
        BuildingKind::GoldMine | BuildingKind::Farm => "\nActions: aucune".to_string(),
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

fn unique_unit_cost(faction: Faction) -> &'static str {
    match faction {
        Faction::Valdorian => "75n/55o",
        Faction::Kharzun => "70n/65o",
        Faction::Sylvans => "55n/45o",
        Faction::Necrarchs => "25o",
    }
}
