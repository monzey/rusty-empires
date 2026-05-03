use crate::core::geometry::distance;
use crate::core::rules::{economy, recruitment, turns};
use crate::core::{BuildingKind, Camp, Event, Game, GridPosition, TurnError};

pub(crate) fn run_ai_turn(game: &mut Game) -> Result<Vec<Event>, TurnError> {
    if game.current_turn != Camp::Ai {
        return Err(TurnError::NotAiTurn);
    }

    let mut events = Vec::new();

    if let Some(barracks_position) = empty_ai_barracks(game) {
        if let Ok(recruitment_events) = recruitment::recruit_soldier(game, barracks_position) {
            events.extend(recruitment_events);
        }
    }

    if let (Some(ai_index), Some(target)) = (
        game.units.iter().position(|unit| unit.camp == Camp::Ai),
        game.villager_position(Camp::Human),
    ) {
        let occupied_positions: Vec<GridPosition> = game
            .units
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != ai_index)
            .map(|(_, unit)| unit.position)
            .collect();
        let unit_id = game.units[ai_index].id;
        let from = game.units[ai_index].position;
        let camp = game.units[ai_index].camp;
        let enemy_building_positions: Vec<GridPosition> = game
            .buildings
            .iter()
            .filter(|building| building.camp != camp)
            .map(|building| building.position)
            .collect();

        if !game.units[ai_index].has_acted {
            if let Some(to) = next_move(
                from,
                target,
                &occupied_positions,
                &enemy_building_positions,
                game.units[ai_index].move_range,
                game.map_width,
                game.map_height,
            ) {
                game.units[ai_index].position = to;
                events.push(Event::UnitMoved { unit_id, from, to });
            }

            game.units[ai_index].has_acted = true;
            events.push(Event::UnitActed { unit_id });
        }
    }

    events.extend(economy::produce_end_of_turn_resources(game, Camp::Ai));
    let from = game.current_turn;
    game.current_turn = Camp::Human;
    turns::reset_units_for_turn(game, Camp::Human);
    events.push(Event::TurnChanged {
        from,
        to: game.current_turn,
    });

    Ok(events)
}

fn empty_ai_barracks(game: &Game) -> Option<GridPosition> {
    game.buildings
        .iter()
        .find(|building| {
            building.camp == Camp::Ai
                && building.kind == BuildingKind::Barracks
                && !game
                    .units
                    .iter()
                    .any(|unit| unit.position == building.position)
        })
        .map(|building| building.position)
}

fn next_move(
    from: GridPosition,
    target: GridPosition,
    occupied_positions: &[GridPosition],
    enemy_building_positions: &[GridPosition],
    move_range: i32,
    map_width: i32,
    map_height: i32,
) -> Option<GridPosition> {
    let mut candidates = Vec::new();

    for y in 0..map_height {
        for x in 0..map_width {
            let position = GridPosition { x, y };

            if position != from
                && distance(from, position) <= move_range
                && !occupied_positions.contains(&position)
                && !enemy_building_positions.contains(&position)
            {
                candidates.push(position);
            }
        }
    }

    candidates.sort_by_key(|position| distance(*position, target));
    candidates.into_iter().next()
}
