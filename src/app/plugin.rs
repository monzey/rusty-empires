use bevy::{prelude::*, window::PresentMode};

use super::camera::{move_camera, zoom_camera};
use super::constants::{MAP_HEIGHT, MAP_WIDTH};
use super::hud::{update_context_menu, update_hud};
use super::input::{
    handle_build_input, handle_context_menu_input, handle_deselect_input, handle_end_turn_input,
    handle_faction_selection_input, handle_human_input, run_ai_turn,
};
use super::resources::{ContextMenu, FactionSelection, GameState, SelectedBuilding, SelectedUnit};
use super::setup::setup;
use super::visuals::{update_building_visuals, update_tile_visuals, update_unit_visuals};
use crate::Game;

pub struct RustyEmpiresAppPlugin;

impl Plugin for RustyEmpiresAppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
            .insert_resource(GameState(Game::new_single_player_vs_ai(
                MAP_WIDTH, MAP_HEIGHT,
            )))
            .insert_resource(FactionSelection::default())
            .insert_resource(SelectedUnit::default())
            .insert_resource(SelectedBuilding::default())
            .insert_resource(ContextMenu::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusty Empires".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    handle_faction_selection_input,
                    handle_context_menu_input,
                    handle_human_input,
                    handle_deselect_input,
                    handle_build_input,
                    handle_end_turn_input,
                    move_camera,
                    zoom_camera,
                    run_ai_turn,
                    update_tile_visuals,
                    update_unit_visuals,
                    update_building_visuals,
                    update_hud,
                    update_context_menu,
                ),
            );
    }
}
