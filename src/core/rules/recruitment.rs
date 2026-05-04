use crate::core::units::{
    UnitState, ARCHER_ATTACK, ARCHER_ATTACK_RANGE, ARCHER_DEFENSE, ARCHER_HEALTH,
    ARCHER_MOVE_RANGE, ELYR_PATHFINDER_ATTACK, ELYR_PATHFINDER_ATTACK_RANGE,
    ELYR_PATHFINDER_DEFENSE, ELYR_PATHFINDER_HEALTH, ELYR_PATHFINDER_MOVE_RANGE,
    KHARZUN_SHIELDBREAKER_ATTACK, KHARZUN_SHIELDBREAKER_ATTACK_RANGE,
    KHARZUN_SHIELDBREAKER_DEFENSE, KHARZUN_SHIELDBREAKER_HEALTH, KHARZUN_SHIELDBREAKER_MOVE_RANGE,
    OBSIDIAN_BONE_SERVANT_ATTACK, OBSIDIAN_BONE_SERVANT_ATTACK_RANGE,
    OBSIDIAN_BONE_SERVANT_DEFENSE, OBSIDIAN_BONE_SERVANT_HEALTH, OBSIDIAN_BONE_SERVANT_MOVE_RANGE,
    SOLDIER_ATTACK, SOLDIER_ATTACK_RANGE, SOLDIER_DEFENSE, SOLDIER_HEALTH, SOLDIER_MOVE_RANGE,
    VALDORIAN_LEGIONARY_ATTACK, VALDORIAN_LEGIONARY_ATTACK_RANGE, VALDORIAN_LEGIONARY_DEFENSE,
    VALDORIAN_LEGIONARY_HEALTH, VALDORIAN_LEGIONARY_MOVE_RANGE, VILLAGER_ATTACK,
    VILLAGER_ATTACK_RANGE, VILLAGER_DEFENSE, VILLAGER_HEALTH, VILLAGER_MOVE_RANGE,
};
use crate::core::{
    BuildingKind, Camp, Event, Faction, Game, GridPosition, RecruitError, UnitId, UnitKind,
};

const SOLDIER_FOOD_COST: i32 = 10;
const ARCHER_FOOD_COST: i32 = 10;
const ARCHER_GOLD_COST: i32 = 5;
const VILLAGER_FOOD_COST: i32 = 10;

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

pub(crate) fn recruit_unique_unit(
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

    let spec = unique_unit_spec(game.faction(camp));
    let stockpile = resources(game, camp);
    if stockpile.food < spec.food_cost {
        return Err(RecruitError::NotEnoughFood);
    }

    if stockpile.gold < spec.gold_cost {
        return Err(RecruitError::NotEnoughGold);
    }

    {
        let resources = resources_mut(game, camp);
        resources.food -= spec.food_cost;
        resources.gold -= spec.gold_cost;
    }

    let unit_id = UnitId(game.next_unit_id);
    game.next_unit_id += 1;
    game.units.push(UnitState {
        id: unit_id,
        kind: spec.kind,
        camp,
        position: building_position,
        has_moved: false,
        has_acted: false,
        health: spec.health,
        attack: spec.attack,
        defense: spec.defense,
        attack_range: spec.attack_range,
        move_range: spec.move_range,
    });

    Ok(vec![Event::UnitRecruited {
        unit_id,
        camp,
        kind: spec.kind,
        position: building_position,
    }])
}

pub(crate) fn recruit_archer(
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

    let stockpile = resources(game, camp);
    if stockpile.food < ARCHER_FOOD_COST {
        return Err(RecruitError::NotEnoughFood);
    }

    if stockpile.gold < ARCHER_GOLD_COST {
        return Err(RecruitError::NotEnoughGold);
    }

    {
        let resources = resources_mut(game, camp);
        resources.food -= ARCHER_FOOD_COST;
        resources.gold -= ARCHER_GOLD_COST;
    }

    let unit_id = UnitId(game.next_unit_id);
    game.next_unit_id += 1;
    game.units.push(UnitState {
        id: unit_id,
        kind: UnitKind::Archer,
        camp,
        position: building_position,
        has_moved: false,
        has_acted: false,
        health: ARCHER_HEALTH,
        attack: ARCHER_ATTACK,
        defense: ARCHER_DEFENSE,
        attack_range: ARCHER_ATTACK_RANGE,
        move_range: ARCHER_MOVE_RANGE,
    });

    Ok(vec![Event::UnitRecruited {
        unit_id,
        camp,
        kind: UnitKind::Archer,
        position: building_position,
    }])
}

pub(crate) fn recruit_villager(
    game: &mut Game,
    building_position: GridPosition,
) -> Result<Vec<Event>, RecruitError> {
    let Some((camp, BuildingKind::Forum)) = game.building_at(building_position) else {
        return Err(RecruitError::NoForum);
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

    if resources(game, camp).food < VILLAGER_FOOD_COST {
        return Err(RecruitError::NotEnoughFood);
    }

    resources_mut(game, camp).food -= VILLAGER_FOOD_COST;
    let unit_id = UnitId(game.next_unit_id);
    game.next_unit_id += 1;
    game.units.push(UnitState {
        id: unit_id,
        kind: UnitKind::Villager,
        camp,
        position: building_position,
        has_moved: false,
        has_acted: false,
        health: VILLAGER_HEALTH,
        attack: VILLAGER_ATTACK,
        defense: VILLAGER_DEFENSE,
        attack_range: VILLAGER_ATTACK_RANGE,
        move_range: VILLAGER_MOVE_RANGE,
    });

    Ok(vec![Event::UnitRecruited {
        unit_id,
        camp,
        kind: UnitKind::Villager,
        position: building_position,
    }])
}

fn resources(game: &Game, camp: Camp) -> crate::core::resources::ResourceStockpile {
    match camp {
        Camp::Human => game.human_resources,
        Camp::Ai => game.ai_resources,
    }
}

#[derive(Debug, Clone, Copy)]
struct UniqueUnitSpec {
    kind: UnitKind,
    food_cost: i32,
    gold_cost: i32,
    health: i32,
    attack: i32,
    defense: i32,
    attack_range: i32,
    move_range: i32,
}

fn unique_unit_spec(faction: Faction) -> UniqueUnitSpec {
    match faction {
        Faction::Valdorian => UniqueUnitSpec {
            kind: UnitKind::ValdorianLegionary,
            food_cost: 75,
            gold_cost: 55,
            health: VALDORIAN_LEGIONARY_HEALTH,
            attack: VALDORIAN_LEGIONARY_ATTACK,
            defense: VALDORIAN_LEGIONARY_DEFENSE,
            attack_range: VALDORIAN_LEGIONARY_ATTACK_RANGE,
            move_range: VALDORIAN_LEGIONARY_MOVE_RANGE,
        },
        Faction::Kharzun => UniqueUnitSpec {
            kind: UnitKind::KharzunShieldbreaker,
            food_cost: 70,
            gold_cost: 65,
            health: KHARZUN_SHIELDBREAKER_HEALTH,
            attack: KHARZUN_SHIELDBREAKER_ATTACK,
            defense: KHARZUN_SHIELDBREAKER_DEFENSE,
            attack_range: KHARZUN_SHIELDBREAKER_ATTACK_RANGE,
            move_range: KHARZUN_SHIELDBREAKER_MOVE_RANGE,
        },
        Faction::Sylvans => UniqueUnitSpec {
            kind: UnitKind::ElyrPathfinder,
            food_cost: 55,
            gold_cost: 45,
            health: ELYR_PATHFINDER_HEALTH,
            attack: ELYR_PATHFINDER_ATTACK,
            defense: ELYR_PATHFINDER_DEFENSE,
            attack_range: ELYR_PATHFINDER_ATTACK_RANGE,
            move_range: ELYR_PATHFINDER_MOVE_RANGE,
        },
        Faction::Necrarchs => UniqueUnitSpec {
            kind: UnitKind::ObsidianBoneServant,
            food_cost: 0,
            gold_cost: 25,
            health: OBSIDIAN_BONE_SERVANT_HEALTH,
            attack: OBSIDIAN_BONE_SERVANT_ATTACK,
            defense: OBSIDIAN_BONE_SERVANT_DEFENSE,
            attack_range: OBSIDIAN_BONE_SERVANT_ATTACK_RANGE,
            move_range: OBSIDIAN_BONE_SERVANT_MOVE_RANGE,
        },
    }
}

fn resources_mut(game: &mut Game, camp: Camp) -> &mut crate::core::resources::ResourceStockpile {
    match camp {
        Camp::Human => &mut game.human_resources,
        Camp::Ai => &mut game.ai_resources,
    }
}
