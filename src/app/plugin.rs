use bevy::{prelude::*, window::PresentMode};

use super::action_bar::handle_action_bar_buttons;
use super::camera::{move_camera, zoom_camera};
use super::constants::{MAP_HEIGHT, MAP_WIDTH};
use super::context_menu::{
    handle_context_menu_action_button, handle_context_menu_input, update_context_menu,
    PendingContextAction,
};
use super::hud::update_top_bar;
use super::input::{
    handle_build_input, handle_end_turn_input, handle_faction_selection_input, handle_human_input,
    run_ai_turn,
};
use super::map_view::update_tile_viewport;
use super::resources::{
    ActiveTiles, ContextMenu, FactionSelection, GameState, SelectedBuilding, SelectedUnit,
};
use super::selection::handle_deselect_input;
use super::selection_panel::update_selection_panel;
use super::setup::setup;
use super::tooltip::update_hover_tooltip;
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
            .insert_resource(PendingContextAction::default())
            .insert_resource(ActiveTiles::default())
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
                    handle_context_menu_action_button,
                    handle_human_input,
                    handle_deselect_input,
                    handle_build_input,
                    handle_action_bar_buttons,
                    handle_end_turn_input,
                    move_camera,
                    zoom_camera,
                    run_ai_turn,
                    update_tile_viewport,
                    update_tile_visuals,
                    update_unit_visuals,
                    update_building_visuals,
                    update_top_bar.run_if(top_bar_needs_update),
                    update_selection_panel.run_if(selection_panel_needs_update),
                    update_context_menu.run_if(context_menu_needs_update),
                    update_hover_tooltip,
                ),
            );
    }
}

fn top_bar_needs_update(game: Res<GameState>, faction_selection: Res<FactionSelection>) -> bool {
    game.is_changed() || faction_selection.is_changed()
}

fn selection_panel_needs_update(
    game: Res<GameState>,
    faction_selection: Res<FactionSelection>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
) -> bool {
    game.is_changed()
        || faction_selection.is_changed()
        || selected_unit.is_changed()
        || selected_building.is_changed()
}

fn context_menu_needs_update(
    context_menu: Res<ContextMenu>,
    pending_action: Res<PendingContextAction>,
) -> bool {
    context_menu.is_changed() || pending_action.is_changed()
}
