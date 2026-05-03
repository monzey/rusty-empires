use crate::core::geometry::distance;
use crate::core::{CombatError, Event, Game, UnitId};

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
