use crate::core::{BuildingKind, Camp, Event, Game, ResearchError, UnitKind};

const MILITARY_TRAINING_COST: i32 = 10;
const MILITARY_TRAINING_ATTACK_BONUS: i32 = 1;
const MILITARY_TRAINING_NAME: &str = "MilitaryTraining";
const AGRICULTURE_TECHNOLOGY_COST: i32 = 80;
const AGRICULTURE_FOOD_COST: i32 = 60;
const AGRICULTURE_NAME: &str = "Agriculture";

pub(crate) fn research_agriculture(game: &mut Game) -> Result<Vec<Event>, ResearchError> {
    let camp = game.current_turn;

    if !has_university(game, camp) {
        return Err(ResearchError::NoUniversity);
    }

    if !has_farm(game, camp) {
        return Err(ResearchError::NoFarm);
    }

    if agriculture_researched(game, camp) {
        return Err(ResearchError::AlreadyResearched);
    }

    if resources(game, camp).technology_points < AGRICULTURE_TECHNOLOGY_COST {
        return Err(ResearchError::NotEnoughTechnologyPoints);
    }

    if resources(game, camp).food < AGRICULTURE_FOOD_COST {
        return Err(ResearchError::NotEnoughFood);
    }

    {
        let resources = resources_mut(game, camp);
        resources.technology_points -= AGRICULTURE_TECHNOLOGY_COST;
        resources.food -= AGRICULTURE_FOOD_COST;
    }

    match camp {
        Camp::Human => game.human_agriculture_researched = true,
        Camp::Ai => game.ai_agriculture_researched = true,
    }

    Ok(vec![Event::TechnologyResearched {
        camp,
        name: AGRICULTURE_NAME,
    }])
}

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

fn has_farm(game: &Game, camp: Camp) -> bool {
    game.buildings
        .iter()
        .any(|building| building.camp == camp && building.kind == BuildingKind::Farm)
}

fn agriculture_researched(game: &Game, camp: Camp) -> bool {
    match camp {
        Camp::Human => game.human_agriculture_researched,
        Camp::Ai => game.ai_agriculture_researched,
    }
}

fn resources(game: &Game, camp: Camp) -> crate::core::resources::ResourceStockpile {
    match camp {
        Camp::Human => game.human_resources,
        Camp::Ai => game.ai_resources,
    }
}

fn resources_mut(game: &mut Game, camp: Camp) -> &mut crate::core::resources::ResourceStockpile {
    match camp {
        Camp::Human => &mut game.human_resources,
        Camp::Ai => &mut game.ai_resources,
    }
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
