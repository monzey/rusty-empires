use bevy::prelude::*;

use super::components::{Building, HudText, MapPosition, Unit};
use super::resources::{GameState, SelectedBuilding, SelectedUnit};
use crate::Camp;

pub(super) fn update_hud(
    game: Res<GameState>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
    units: Query<(&Unit, &MapPosition)>,
    buildings: Query<(&Building, &MapPosition)>,
    mut hud: Query<&mut Text, With<HudText>>,
) {
    let Ok(mut text) = hud.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!(
        "Tour: {:?}\nOr: {}\nNourriture: {}\nTech: {}{}\n{}",
        game.0.current_turn(),
        game.0.gold(Camp::Human),
        game.0.food(Camp::Human),
        game.0.technology_points(Camp::Human),
        winner_text(&game),
        selection_text(
            &game,
            &selected_unit,
            &selected_building,
            &units,
            &buildings
        ),
    );
}

fn winner_text(game: &GameState) -> String {
    match game.0.winner() {
        Some(winner) => format!("\nVictoire: {:?}", winner),
        None => String::new(),
    }
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
                "Selection: {:?} {:?} ({}, {})\nPV: {} | Bouge: {} | Agi: {}",
                unit.camp,
                unit.kind,
                position.0.x,
                position.0.y,
                game.0.unit_health(unit.id).unwrap_or(0),
                game.0.unit_has_moved(unit.id).unwrap_or(false),
                game.0.unit_has_acted(unit.id).unwrap_or(false),
            );
        }
    }

    if let Some(entity) = selected_building.0 {
        if let Ok((building, position)) = buildings.get(entity) {
            return format!(
                "Selection: {:?} {:?} ({}, {})",
                building.camp, building.kind, position.0.x, position.0.y,
            );
        }
    }

    "Selection: aucune".to_string()
}
