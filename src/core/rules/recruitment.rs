use crate::core::units::{
    UnitState, SOLDIER_ATTACK, SOLDIER_ATTACK_RANGE, SOLDIER_DEFENSE, SOLDIER_HEALTH,
    SOLDIER_MOVE_RANGE,
};
use crate::core::{BuildingKind, Camp, Event, Game, GridPosition, RecruitError, UnitId, UnitKind};

const SOLDIER_FOOD_COST: i32 = 10;

pub(crate) fn recruit_soldier(
    game: &mut Game,
    building_position: GridPosition,
) -> Result<Vec<Event>, RecruitError> {
    let Some((camp, BuildingKind::Barracks)) = game.building_at(building_position) else {
        return Err(RecruitError::NoBarracks);
    };

    if camp != game.current_turn {
        return Err(RecruitError::NotOwnerTurn);
    }

    if game
        .units
        .iter()
        .any(|unit| unit.position == building_position)
    {
        return Err(RecruitError::Occupied);
    }

    if resources(game, camp).food < SOLDIER_FOOD_COST {
        return Err(RecruitError::NotEnoughFood);
    }

    resources_mut(game, camp).food -= SOLDIER_FOOD_COST;
    let unit_id = UnitId(game.next_unit_id);
    game.next_unit_id += 1;
    game.units.push(UnitState {
        id: unit_id,
        kind: UnitKind::Soldier,
        camp,
        position: building_position,
        has_moved: false,
        has_acted: false,
        health: SOLDIER_HEALTH,
        attack: SOLDIER_ATTACK,
        defense: SOLDIER_DEFENSE,
        attack_range: SOLDIER_ATTACK_RANGE,
        move_range: SOLDIER_MOVE_RANGE,
    });

    Ok(vec![Event::UnitRecruited {
        unit_id,
        camp,
        kind: UnitKind::Soldier,
        position: building_position,
    }])
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
