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
    mut panel: Query<(&mut Text, &mut Style), With<SelectionPanelText>>,
) {
    let Ok((mut text, mut style)) = panel.get_single_mut() else {
        return;
    };

    if faction_selection.0 {
        style.display = Display::None;
        return;
    }

    let Some(selection) = selection_text(
        &game,
        &selected_unit,
        &selected_building,
        &units,
        &buildings,
    ) else {
        style.display = Display::None;
        return;
    };

    style.display = Display::Flex;
    text.sections[0].value = selection;
}

fn selection_text(
    game: &GameState,
    selected_unit: &SelectedUnit,
    selected_building: &SelectedBuilding,
    units: &Query<(&Unit, &MapPosition)>,
    buildings: &Query<(&Building, &MapPosition)>,
) -> Option<String> {
    if let Some(entity) = selected_unit.0 {
        if let Ok((unit, position)) = units.get(entity) {
            return Some(unit_selection_text(game, unit, position));
        }
    }

    if let Some(entity) = selected_building.0 {
        if let Ok((building, position)) = buildings.get(entity) {
            return Some(building_selection_text(game, building, position));
        }
    }

    None
}

fn unit_selection_text(game: &GameState, unit: &Unit, position: &MapPosition) -> String {
    let mut lines = vec![
        format!("Selection: {:?} {:?}", unit.camp, unit.kind),
        format!("Position: {}, {}", position.0.x, position.0.y),
        format!(
            "Stats: PV {} | ATQ {} | DEF {} | Portee {} | Mouv {}",
            game.0.unit_health(unit.id).unwrap_or(0),
            game.0.unit_attack(unit.id).unwrap_or(0),
            game.0.unit_defense(unit.id).unwrap_or(0),
            game.0.unit_attack_range(unit.id).unwrap_or(0),
            game.0.unit_move_range(unit.id).unwrap_or(0),
        ),
        format!(
            "Etat: bouge {} | agi {}",
            game.0.unit_has_moved(unit.id).unwrap_or(false),
            game.0.unit_has_acted(unit.id).unwrap_or(false),
        ),
        "Actions disponibles:".to_string(),
    ];
    lines.extend(
        unit_actions(unit.kind)
            .into_iter()
            .map(|line| line.to_string()),
    );
    lines.join("\n")
}

fn building_selection_text(
    game: &GameState,
    building: &Building,
    position: &MapPosition,
) -> String {
    let mut lines = vec![
        format!("Selection: {:?} {:?}", building.camp, building.kind),
        format!("Position: {}, {}", position.0.x, position.0.y),
        format!(
            "Stats: PV {} | Agi {}",
            game.0.building_health(position.0).unwrap_or(0),
            game.0.building_has_acted(position.0).unwrap_or(false),
        ),
        "Actions disponibles:".to_string(),
    ];
    lines.extend(building_actions(
        building.kind,
        game.0.faction(building.camp),
    ));
    lines.join("\n")
}

fn unit_actions(kind: UnitKind) -> Vec<&'static str> {
    match kind {
        UnitKind::Villager => vec![
            "Clic case: bouger",
            "Clic ennemi: attaquer",
            "B Mine 60o | F Ferme 40o | T Forum 250o/100n",
            "R Caserne 180o/80n | M Marche 160o/80n",
            "U Universite 220o/120n | O Tour 140o/40n",
        ],
        UnitKind::Soldier
        | UnitKind::Archer
        | UnitKind::ValdorianLegionary
        | UnitKind::KharzunShieldbreaker
        | UnitKind::ElyrPathfinder
        | UnitKind::ObsidianBoneServant => {
            vec!["Clic case: bouger", "Clic ennemi: attaquer"]
        }
    }
}

fn building_actions(kind: BuildingKind, faction: Faction) -> Vec<String> {
    match kind {
        BuildingKind::Forum => vec!["V Villageois 10n".to_string()],
        BuildingKind::Barracks => vec![format!(
            "S Soldat 10n | A Archer 10n/5o | C {} {}",
            unique_unit_label(faction),
            unique_unit_cost(faction),
        )],
        BuildingKind::Market => vec!["G 10o -> 5n | N 10n -> 5o".to_string()],
        BuildingKind::University => vec![
            "H Agriculture 80t/60n".to_string(),
            "Y Entrainement militaire 10t".to_string(),
        ],
        BuildingKind::Watchtower => vec!["Clic unite ennemie: tir portee 2-5".to_string()],
        BuildingKind::GoldMine | BuildingKind::Farm => vec!["Aucune action directe".to_string()],
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
