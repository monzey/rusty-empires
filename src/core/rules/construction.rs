use crate::core::buildings::{health_for_kind, BuildingState};
use crate::core::geometry::distance;
use crate::core::resources::ResourceStockpile;
use crate::core::{BuildError, BuildingKind, Event, Game, NaturalResource, UnitId, UnitKind};

pub(crate) fn build_gold_mine(game: &mut Game, unit_id: UnitId) -> Result<Vec<Event>, BuildError> {
    build_on_resource(
        game,
        unit_id,
        NaturalResource::GoldDeposit,
        BuildingKind::GoldMine,
        BuildError::NoGoldDeposit,
    )
}

pub(crate) fn build_farm(game: &mut Game, unit_id: UnitId) -> Result<Vec<Event>, BuildError> {
    build_on_resource(
        game,
        unit_id,
        NaturalResource::Field,
        BuildingKind::Farm,
        BuildError::NoField,
    )
}

pub(crate) fn build_forum(game: &mut Game, unit_id: UnitId) -> Result<Vec<Event>, BuildError> {
    build_non_resource_building(game, unit_id, BuildingKind::Forum, false)
}

pub(crate) fn build_barracks(game: &mut Game, unit_id: UnitId) -> Result<Vec<Event>, BuildError> {
    build_non_resource_building(game, unit_id, BuildingKind::Barracks, true)
}

pub(crate) fn build_market(game: &mut Game, unit_id: UnitId) -> Result<Vec<Event>, BuildError> {
    build_non_resource_building(game, unit_id, BuildingKind::Market, true)
}

pub(crate) fn build_university(game: &mut Game, unit_id: UnitId) -> Result<Vec<Event>, BuildError> {
    build_non_resource_building(game, unit_id, BuildingKind::University, true)
}

pub(crate) fn build_watchtower(game: &mut Game, unit_id: UnitId) -> Result<Vec<Event>, BuildError> {
    build_non_resource_building(game, unit_id, BuildingKind::Watchtower, false)
}

fn build_on_resource(
    game: &mut Game,
    unit_id: UnitId,
    required_resource: NaturalResource,
    building_kind: BuildingKind,
    missing_resource_error: BuildError,
) -> Result<Vec<Event>, BuildError> {
    let unit_index = game
        .units
        .iter()
        .position(|unit| unit.id == unit_id)
        .ok_or(BuildError::NoUnit)?;

    let unit = &game.units[unit_index];
    if unit.kind != UnitKind::Villager {
        return Err(BuildError::NotBuilder);
    }

    if unit.camp != game.current_turn {
        return Err(BuildError::NotUnitTurn);
    }

    if unit.has_acted {
        return Err(BuildError::AlreadyActed);
    }

    let position = unit.position;
    if game.natural_resource_at(position) != Some(required_resource) {
        return Err(missing_resource_error);
    }

    if game
        .buildings
        .iter()
        .any(|building| building.position == position)
    {
        return Err(BuildError::OccupiedByBuilding);
    }

    let camp = unit.camp;
    pay_construction_cost(game, camp, building_kind)?;
    game.buildings.push(BuildingState {
        camp,
        kind: building_kind,
        position,
        health: health_for_kind(building_kind),
        has_acted: false,
    });
    game.units[unit_index].has_acted = true;

    Ok(vec![
        Event::BuildingConstructed {
            camp,
            kind: building_kind,
            position,
        },
        Event::UnitActed { unit_id },
    ])
}

fn build_non_resource_building(
    game: &mut Game,
    unit_id: UnitId,
    building_kind: BuildingKind,
    requires_adjacent_forum: bool,
) -> Result<Vec<Event>, BuildError> {
    let unit_index = game
        .units
        .iter()
        .position(|unit| unit.id == unit_id)
        .ok_or(BuildError::NoUnit)?;

    let unit = &game.units[unit_index];
    if unit.kind != UnitKind::Villager {
        return Err(BuildError::NotBuilder);
    }

    if unit.camp != game.current_turn {
        return Err(BuildError::NotUnitTurn);
    }

    if unit.has_acted {
        return Err(BuildError::AlreadyActed);
    }

    let position = unit.position;
    if game.natural_resource_at(position).is_some() {
        return Err(BuildError::NaturalResourcePresent);
    }

    if game
        .buildings
        .iter()
        .any(|building| building.position == position)
    {
        return Err(BuildError::OccupiedByBuilding);
    }

    let camp = unit.camp;
    if building_kind == BuildingKind::Watchtower && !is_in_watchtower_cross(game, camp, position) {
        return Err(BuildError::NoWatchtowerCross);
    }

    if requires_adjacent_forum && !has_adjacent_forum(game, camp, position) {
        return Err(BuildError::NoAdjacentForum);
    }

    pay_construction_cost(game, camp, building_kind)?;
    game.buildings.push(BuildingState {
        camp,
        kind: building_kind,
        position,
        health: health_for_kind(building_kind),
        has_acted: false,
    });
    game.units[unit_index].has_acted = true;

    Ok(vec![
        Event::BuildingConstructed {
            camp,
            kind: building_kind,
            position,
        },
        Event::UnitActed { unit_id },
    ])
}

#[derive(Debug, Clone, Copy)]
struct ConstructionCost {
    gold: i32,
    food: i32,
}

fn pay_construction_cost(
    game: &mut Game,
    camp: crate::core::Camp,
    building_kind: BuildingKind,
) -> Result<(), BuildError> {
    let cost = construction_cost(building_kind);
    let stockpile = resources(game, camp);

    if stockpile.gold < cost.gold {
        return Err(BuildError::NotEnoughGold);
    }

    if stockpile.food < cost.food {
        return Err(BuildError::NotEnoughFood);
    }

    let stockpile = resources_mut(game, camp);
    stockpile.gold -= cost.gold;
    stockpile.food -= cost.food;
    Ok(())
}

fn construction_cost(building_kind: BuildingKind) -> ConstructionCost {
    match building_kind {
        BuildingKind::GoldMine => ConstructionCost { gold: 60, food: 0 },
        BuildingKind::Farm => ConstructionCost { gold: 40, food: 0 },
        BuildingKind::Forum => ConstructionCost {
            gold: 250,
            food: 100,
        },
        BuildingKind::Barracks => ConstructionCost {
            gold: 180,
            food: 80,
        },
        BuildingKind::Market => ConstructionCost {
            gold: 160,
            food: 80,
        },
        BuildingKind::University => ConstructionCost {
            gold: 220,
            food: 120,
        },
        BuildingKind::Watchtower => ConstructionCost {
            gold: 140,
            food: 40,
        },
    }
}

fn resources(game: &Game, camp: crate::core::Camp) -> ResourceStockpile {
    match camp {
        crate::core::Camp::Human => game.human_resources,
        crate::core::Camp::Ai => game.ai_resources,
    }
}

fn resources_mut(game: &mut Game, camp: crate::core::Camp) -> &mut ResourceStockpile {
    match camp {
        crate::core::Camp::Human => &mut game.human_resources,
        crate::core::Camp::Ai => &mut game.ai_resources,
    }
}

fn has_adjacent_forum(
    game: &Game,
    camp: crate::core::Camp,
    position: crate::core::GridPosition,
) -> bool {
    game.buildings.iter().any(|building| {
        building.camp == camp
            && building.kind == BuildingKind::Forum
            && distance(building.position, position) == 1
    })
}

fn is_in_watchtower_cross(
    game: &Game,
    camp: crate::core::Camp,
    position: crate::core::GridPosition,
) -> bool {
    game.buildings.iter().any(|building| {
        building.camp == camp
            && building.kind == BuildingKind::Forum
            && distance(building.position, position) == 2
            && (building.position.x == position.x || building.position.y == position.y)
    })
}
