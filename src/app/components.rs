use bevy::prelude::*;

use crate::{BuildingKind, Camp, GridPosition, UnitId};

#[derive(Component)]
pub(super) struct Tile;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(super) struct MapPosition(pub(super) GridPosition);

#[derive(Component)]
pub(super) struct Unit {
    pub(super) id: UnitId,
    pub(super) camp: Camp,
    pub(super) kind: UnitKind,
}

#[derive(Component)]
pub(super) struct Building {
    pub(super) _camp: Camp,
    pub(super) _kind: BuildingKind,
}

#[derive(Clone, Copy)]
pub(super) enum UnitKind {
    Villager,
}
