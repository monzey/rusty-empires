use bevy::prelude::*;

use crate::Game;

#[derive(Resource)]
pub(super) struct GameState(pub(super) Game);

#[derive(Resource)]
pub(super) struct FactionSelection(pub(super) bool);

impl Default for FactionSelection {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource, Default)]
pub(super) struct SelectedUnit(pub(super) Option<Entity>);

#[derive(Resource, Default)]
pub(super) struct SelectedBuilding(pub(super) Option<Entity>);

#[derive(Resource, Default)]
pub(super) struct ContextMenu {
    pub(super) screen_position: Vec2,
    pub(super) lines: Vec<String>,
}

#[derive(Resource, Clone)]
pub(super) struct AppMeshes {
    pub(super) unit: Handle<Mesh>,
    pub(super) building: Handle<Mesh>,
}
