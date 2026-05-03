use crate::core::buildings::BuildingState;
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
    game.buildings.push(BuildingState {
        camp,
        kind: building_kind,
        position,
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
