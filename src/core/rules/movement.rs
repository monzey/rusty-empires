use crate::core::geometry::distance;
use crate::core::{Event, Game, GridPosition, MoveError, UnitId};

pub(crate) fn move_unit(
    game: &mut Game,
    unit_id: UnitId,
    to: GridPosition,
) -> Result<Vec<Event>, MoveError> {
    if !game.is_inside_map(to) {
        return Err(MoveError::OutsideMap);
    }

    let unit_index = game
        .units
        .iter()
        .position(|unit| unit.id == unit_id)
        .ok_or(MoveError::NoUnit)?;

    let unit = &game.units[unit_index];
    if unit.camp != game.current_turn {
        return Err(MoveError::NotUnitTurn);
    }

    if game.units.iter().any(|unit| unit.position == to) {
        return Err(MoveError::Occupied);
    }

    if let Some((building_camp, _)) = game.building_at(to) {
        if building_camp != unit.camp {
            return Err(MoveError::EnemyBuilding);
        }
    }

    if unit.has_acted || unit.has_moved {
        return Err(MoveError::AlreadyActed);
    }

    if distance(unit.position, to) > unit.move_range {
        return Err(MoveError::OutOfRange);
    }

    let from = unit.position;
    game.units[unit_index].position = to;
    game.units[unit_index].has_moved = true;

    Ok(vec![Event::UnitMoved { unit_id, from, to }])
}
