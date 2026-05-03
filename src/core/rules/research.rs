use crate::core::{BuildingKind, Camp, Event, Game, ResearchError, UnitKind};

const MILITARY_TRAINING_COST: i32 = 10;
const MILITARY_TRAINING_ATTACK_BONUS: i32 = 1;
const MILITARY_TRAINING_NAME: &str = "MilitaryTraining";

pub(crate) fn research_military_training(game: &mut Game) -> Result<Vec<Event>, ResearchError> {
    let camp = game.current_turn;

    if !has_university(game, camp) {
        return Err(ResearchError::NoUniversity);
    }

    if technology_points(game, camp) < MILITARY_TRAINING_COST {
        return Err(ResearchError::NotEnoughTechnologyPoints);
    }

    technology_points_mut(game, camp).technology_points -= MILITARY_TRAINING_COST;
    for unit in &mut game.units {
        if unit.camp == camp && unit.kind == UnitKind::Soldier {
            unit.attack += MILITARY_TRAINING_ATTACK_BONUS;
        }
    }

    Ok(vec![Event::TechnologyResearched {
        camp,
        name: MILITARY_TRAINING_NAME,
    }])
}

fn has_university(game: &Game, camp: Camp) -> bool {
    game.buildings
        .iter()
        .any(|building| building.camp == camp && building.kind == BuildingKind::University)
}

fn technology_points(game: &Game, camp: Camp) -> i32 {
    match camp {
        Camp::Human => game.human_resources.technology_points,
        Camp::Ai => game.ai_resources.technology_points,
    }
}

fn technology_points_mut(
    game: &mut Game,
    camp: Camp,
) -> &mut crate::core::resources::ResourceStockpile {
    match camp {
        Camp::Human => &mut game.human_resources,
        Camp::Ai => &mut game.ai_resources,
    }
}
