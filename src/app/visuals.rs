use bevy::prelude::*;

use super::components::Unit;
use super::resources::{GameState, SelectedUnit};
use crate::{BuildingKind, Camp, UnitKind};

pub(super) fn update_unit_visuals(
    selected_unit: Res<SelectedUnit>,
    game: Res<GameState>,
    mut units: Query<(Entity, &Unit, &mut Sprite)>,
) {
    for (entity, unit, mut sprite) in &mut units {
        let selected = selected_unit.0 == Some(entity);
        let inactive =
            unit.camp != game.0.current_turn() || game.0.unit_has_acted(unit.id).unwrap_or(false);
        sprite.color = unit_color(unit.camp, unit.kind, selected, inactive);
    }
}

pub(super) fn unit_color(camp: Camp, kind: UnitKind, selected: bool, inactive: bool) -> Color {
    if selected {
        return Color::srgb(1.0, 0.92, 0.18);
    }

    if inactive {
        return Color::srgb(0.38, 0.39, 0.42);
    }

    match (camp, kind) {
        (Camp::Human, UnitKind::Villager) => Color::srgb(0.2, 0.55, 1.0),
        (Camp::Human, UnitKind::Soldier) => Color::srgb(0.12, 0.24, 0.95),
        (Camp::Ai, UnitKind::Villager) => Color::srgb(0.95, 0.25, 0.22),
        (Camp::Ai, UnitKind::Soldier) => Color::srgb(0.62, 0.05, 0.08),
    }
}

pub(super) fn building_color(camp: Camp, kind: BuildingKind) -> Color {
    match (camp, kind) {
        (Camp::Human, BuildingKind::GoldMine) => Color::srgb(1.0, 0.72, 0.18),
        (Camp::Ai, BuildingKind::GoldMine) => Color::srgb(0.85, 0.44, 0.12),
        (Camp::Human, BuildingKind::Farm) => Color::srgb(0.65, 0.86, 0.28),
        (Camp::Ai, BuildingKind::Farm) => Color::srgb(0.45, 0.67, 0.22),
        (Camp::Human, BuildingKind::Forum) => Color::srgb(0.48, 0.7, 1.0),
        (Camp::Ai, BuildingKind::Forum) => Color::srgb(0.82, 0.28, 0.3),
        (Camp::Human, BuildingKind::Barracks) => Color::srgb(0.24, 0.32, 0.78),
        (Camp::Ai, BuildingKind::Barracks) => Color::srgb(0.45, 0.12, 0.14),
    }
}
