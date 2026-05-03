use bevy::prelude::*;

use super::components::{MapPosition, Unit};
use super::grid::grid_to_world;
use crate::Event;

pub(super) fn apply_game_events(
    events: &[Event],
    commands: &mut Commands,
    units: &mut Query<(Entity, &mut MapPosition, &mut Transform, &Unit)>,
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
            _ => {}
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
            _ => {}
        }
    }
}
