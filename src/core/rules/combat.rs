use crate::core::geometry::distance;
use crate::core::{CombatError, Event, Game, GridPosition, UnitId};

pub(crate) fn attack_unit(
    game: &mut Game,
    attacker_id: UnitId,
    target_id: UnitId,
) -> Result<Vec<Event>, CombatError> {
    let attacker_index = game
        .units
        .iter()
        .position(|unit| unit.id == attacker_id)
        .ok_or(CombatError::NoAttacker)?;
    let target_index = game
        .units
        .iter()
        .position(|unit| unit.id == target_id)
        .ok_or(CombatError::NoTarget)?;

    let attacker = &game.units[attacker_index];
    let target = &game.units[target_index];

    if attacker.camp != game.current_turn {
        return Err(CombatError::NotAttackerTurn);
    }

    if attacker.has_acted {
        return Err(CombatError::AlreadyActed);
    }

    if attacker.camp == target.camp {
        return Err(CombatError::FriendlyTarget);
    }

    if !game.is_visible(attacker.camp, target.position) {
        return Err(CombatError::TargetNotVisible);
    }

    if distance(attacker.position, target.position) > attacker.attack_range {
        return Err(CombatError::OutOfRange);
    }

    let damage = (attacker.attack - target.defense).max(1);
    let remaining_health = (target.health - damage).max(0);
    let defeated = remaining_health == 0;

    game.units[attacker_index].has_acted = true;
    game.units[target_index].health = remaining_health;

    let mut events = vec![Event::UnitDamaged {
        unit_id: target_id,
        amount: damage,
        remaining_health,
    }];

    if defeated {
        game.units.remove(target_index);
        events.push(Event::UnitDefeated { unit_id: target_id });
    }

    events.push(Event::UnitActed {
        unit_id: attacker_id,
    });
    if let Some(camp) = game.winner() {
        events.push(Event::GameWon { camp });
    }
    Ok(events)
}

pub(crate) fn attack_building(
    game: &mut Game,
    attacker_id: UnitId,
    target_position: GridPosition,
) -> Result<Vec<Event>, CombatError> {
    let attacker_index = game
        .units
        .iter()
        .position(|unit| unit.id == attacker_id)
        .ok_or(CombatError::NoAttacker)?;
    let building_index = game
        .buildings
        .iter()
        .position(|building| building.position == target_position)
        .ok_or(CombatError::NoTarget)?;

    let attacker = &game.units[attacker_index];
    let building = &game.buildings[building_index];

    if attacker.camp != game.current_turn {
        return Err(CombatError::NotAttackerTurn);
    }

    if attacker.has_acted {
        return Err(CombatError::AlreadyActed);
    }

    if attacker.camp == building.camp {
        return Err(CombatError::FriendlyTarget);
    }

    if !game.is_visible(attacker.camp, building.position) {
        return Err(CombatError::TargetNotVisible);
    }

    if distance(attacker.position, building.position) > attacker.attack_range {
        return Err(CombatError::OutOfRange);
    }

    let damage = attacker.attack;
    let remaining_health = (building.health - damage).max(0);
    let destroyed = remaining_health == 0;

    game.units[attacker_index].has_acted = true;
    game.buildings[building_index].health = remaining_health;

    let camp = game.buildings[building_index].camp;
    let kind = game.buildings[building_index].kind;
    let position = game.buildings[building_index].position;
    let mut events = vec![Event::BuildingDamaged {
        camp,
        kind,
        position,
        amount: damage,
        remaining_health,
    }];

    if destroyed {
        game.buildings.remove(building_index);
        events.push(Event::BuildingDestroyed {
            camp,
            kind,
            position,
        });
    }

    events.push(Event::UnitActed {
        unit_id: attacker_id,
    });
    if let Some(camp) = game.winner() {
        events.push(Event::GameWon { camp });
    }

    Ok(events)
}
