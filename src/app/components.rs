use bevy::prelude::*;

use crate::{BuildingKind, Camp, GridPosition, UnitId, UnitKind};

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
    pub(super) camp: Camp,
    pub(super) kind: BuildingKind,
}

#[derive(Component)]
pub(super) struct SelectionPanelText;

#[derive(Component)]
pub(super) struct TopBarText;

#[derive(Component)]
pub(super) struct ContextMenuText;

#[derive(Component)]
pub(super) struct ContextMenuRoot;

#[derive(Component)]
pub(super) struct ContextMenuActionButton;

#[derive(Component)]
pub(super) struct TooltipText;

#[derive(Component)]
pub(super) struct HoverInfoPanelText;
