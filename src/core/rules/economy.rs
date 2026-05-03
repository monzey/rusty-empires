use crate::core::resources::ResourceStockpile;
use crate::core::{BuildingKind, Camp, Event, Game};

const GOLD_MINE_YIELD: i32 = 10;
const FARM_YIELD: i32 = 10;

pub(crate) fn produce_end_of_turn_resources(game: &mut Game, camp: Camp) -> Vec<Event> {
    let mut events = Vec::new();
    let gold = game
        .buildings
        .iter()
        .filter(|building| building.camp == camp && building.kind == BuildingKind::GoldMine)
        .count() as i32
        * GOLD_MINE_YIELD;

    if gold > 0 {
        let total = {
            let resources = resources_mut(game, camp);
            resources.gold += gold;
            resources.gold
        };

        events.push(Event::GoldProduced {
            camp,
            amount: gold,
            total,
        });
    }

    let food = game
        .buildings
        .iter()
        .filter(|building| building.camp == camp && building.kind == BuildingKind::Farm)
        .count() as i32
        * FARM_YIELD;

    if food > 0 {
        let total = {
            let resources = resources_mut(game, camp);
            resources.food += food;
            resources.food
        };

        events.push(Event::FoodProduced {
            camp,
            amount: food,
            total,
        });
    }

    events
}

pub(crate) fn resources(game: &Game, camp: Camp) -> ResourceStockpile {
    match camp {
        Camp::Human => game.human_resources,
        Camp::Ai => game.ai_resources,
    }
}

fn resources_mut(game: &mut Game, camp: Camp) -> &mut ResourceStockpile {
    match camp {
        Camp::Human => &mut game.human_resources,
        Camp::Ai => &mut game.ai_resources,
    }
}
