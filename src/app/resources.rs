use bevy::prelude::*;

use crate::Game;

#[derive(Resource)]
pub(super) struct GameState(pub(super) Game);

#[derive(Resource, Default)]
pub(super) struct SelectedUnit(pub(super) Option<Entity>);

#[derive(Resource, Default)]
pub(super) struct SelectedBuilding(pub(super) Option<Entity>);
