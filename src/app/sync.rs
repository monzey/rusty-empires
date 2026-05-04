use bevy::{ecs::query::QueryFilter, prelude::*};

use super::components::{Building, MapPosition, Unit};
use super::grid::grid_to_world;
use super::setup::spawn_unit;
use crate::Event;

pub(super) fn apply_game_events<F: QueryFilter>(
    events: &[Event],
    commands: &mut Commands,
    unit_mesh: &Handle<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    units: &mut Query<(Entity, &mut MapPosition, &mut Transform, &Unit), F>,
) {
    for event in events {
        match *event {
            Event::UnitMoved { unit_id, to, .. } => {
                for (_, mut position, mut transform, unit) in units.iter_mut() {
                    if unit.id == unit_id {
                        position.0 = to;
                        transform.translation = grid_to_world(to, 1.0);
                        break;
                    }
                }
            }
            Event::UnitDefeated { unit_id } => {
                if let Some((entity, _, _, _)) =
                    units.iter_mut().find(|(_, _, _, unit)| unit.id == unit_id)
                {
                    commands.entity(entity).despawn();
                }
            }
            Event::UnitRecruited {
                unit_id,
                camp,
                kind,
                position,
            } => {
                spawn_unit(
                    commands, unit_mesh, materials, unit_id, kind, camp, position,
                );
            }
            _ => {}
        }
    }
}

pub(super) fn apply_building_events<F: QueryFilter>(
    events: &[Event],
    commands: &mut Commands,
    buildings: &Query<(Entity, &MapPosition, &Building), F>,
) {
    for event in events {
        if let Event::BuildingDestroyed { position, .. } = *event {
            if let Some((entity, _, _)) = buildings
                .iter()
                .find(|(_, building_position, _)| building_position.0 == position)
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub(super) fn log_resource_events(events: &[Event]) {
    for event in events {
        match *event {
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
            Event::GoldTradedForFood {
                camp,
                gold_spent,
                food_gained,
                gold_total,
                food_total,
            } => {
                info!(
                    "{:?} echange {} or contre {} nourriture (or: {}, nourriture: {}).",
                    camp, gold_spent, food_gained, gold_total, food_total
                );
            }
            Event::FoodTradedForGold {
                camp,
                food_spent,
                gold_gained,
                food_total,
                gold_total,
            } => {
                info!(
                    "{:?} echange {} nourriture contre {} or (nourriture: {}, or: {}).",
                    camp, food_spent, gold_gained, food_total, gold_total
                );
            }
            Event::TechnologyResearched { camp, name } => {
                info!("{:?} recherche {}.", camp, name);
            }
            _ => {}
        }
    }
}

pub(super) fn log_combat_events(events: &[Event]) {
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
