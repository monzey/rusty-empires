use crate::core::rules::economy;
use crate::core::{Camp, Event, Game, TurnError};

pub(crate) fn end_human_turn(game: &mut Game) -> Result<Vec<Event>, TurnError> {
    if game.current_turn != Camp::Human {
        return Err(TurnError::NotHumanTurn);
    }

    let mut events = economy::produce_end_of_turn_resources(game, Camp::Human);
    let from = game.current_turn;
    game.current_turn = Camp::Ai;
    reset_units_for_turn(game, Camp::Ai);
    events.push(Event::TurnChanged {
        from,
        to: game.current_turn,
    });
    Ok(events)
}

pub(crate) fn reset_units_for_turn(game: &mut Game, camp: Camp) {
    for unit in game.units.iter_mut().filter(|unit| unit.camp == camp) {
        unit.has_moved = false;
        unit.has_acted = false;
    }
}
