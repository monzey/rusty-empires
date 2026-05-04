use bevy::prelude::*;

use super::components::{Building, HudText, MapPosition, Tile, Unit};
use super::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use super::grid::grid_to_world;
use super::resources::GameState;
use super::visuals::{building_color, unit_color};
use crate::{BuildingKind, Camp, GridPosition, NaturalResource, UnitId, UnitKind};

pub(super) fn setup(mut commands: Commands, game: Res<GameState>) {
    commands.spawn(Camera2dBundle::default());

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let position = GridPosition { x, y };
            let color = if game.0.is_visible(Camp::Human, position) {
                match game.0.natural_resource_at(position) {
                    Some(NaturalResource::GoldDeposit) => Color::srgb(0.55, 0.42, 0.12),
                    Some(NaturalResource::Field) => Color::srgb(0.42, 0.46, 0.16),
                    None if (x + y) % 2 == 0 => Color::srgb(0.22, 0.31, 0.22),
                    None => Color::srgb(0.18, 0.27, 0.18),
                }
            } else if game.0.is_explored(Camp::Human, position) {
                Color::srgb(0.08, 0.11, 0.1)
            } else {
                Color::srgb(0.01, 0.012, 0.016)
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(grid_to_world(position, 0.0)),
                    ..default()
                },
                Tile,
                MapPosition(position),
            ));
        }
    }

    spawn_unit(
        &mut commands,
        game.0
            .villager_id(Camp::Human)
            .expect("human villager should have an id at game start"),
        UnitKind::Villager,
        Camp::Human,
        game.0
            .villager_position(Camp::Human)
            .expect("human villager should exist at game start"),
    );
    spawn_unit(
        &mut commands,
        game.0
            .villager_id(Camp::Ai)
            .expect("AI villager should have an id at game start"),
        UnitKind::Villager,
        Camp::Ai,
        game.0
            .villager_position(Camp::Ai)
            .expect("AI villager should exist at game start"),
    );
    spawn_unit(
        &mut commands,
        game.0
            .soldier_id(Camp::Human)
            .expect("human soldier should have an id at game start"),
        UnitKind::Soldier,
        Camp::Human,
        game.0
            .soldier_position(Camp::Human)
            .expect("human soldier should exist at game start"),
    );
    spawn_unit(
        &mut commands,
        game.0
            .soldier_id(Camp::Ai)
            .expect("AI soldier should have an id at game start"),
        UnitKind::Soldier,
        Camp::Ai,
        game.0
            .soldier_position(Camp::Ai)
            .expect("AI soldier should exist at game start"),
    );

    spawn_hud(&mut commands);

    info!("Boucle initiale: clic sur une unite pour la selectionner, clic droit/Echap pour deselectionner, clic sur une case libre pour bouger, clic sur une unite ennemie a portee pour attaquer, B mine, F ferme, T forum, R caserne, M marche, U universite, S recruter soldat depuis caserne, A recruter archer depuis caserne, V recruter villageois depuis forum, G echanger or vers nourriture depuis marche, N echanger nourriture vers or depuis marche, Y rechercher entrainement militaire depuis universite, fleches camera, molette zoom, Espace/Entree pour finir le tour.");
}

fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(0.92, 0.94, 0.86),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(12.0),
            ..default()
        }),
        HudText,
    ));
}

pub(super) fn spawn_unit(
    commands: &mut Commands,
    id: UnitId,
    kind: UnitKind,
    camp: Camp,
    grid_position: GridPosition,
) {
    let size = match kind {
        UnitKind::Villager => TILE_SIZE * 0.62,
        UnitKind::Soldier => TILE_SIZE * 0.72,
        UnitKind::Archer => TILE_SIZE * 0.66,
    };
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: unit_color(camp, kind, false, false),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            transform: Transform::from_translation(grid_to_world(grid_position, 1.0)),
            ..default()
        },
        Unit { id, camp, kind },
        MapPosition(grid_position),
    ));
}

pub(super) fn spawn_building(
    commands: &mut Commands,
    camp: Camp,
    kind: BuildingKind,
    grid_position: GridPosition,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: building_color(camp, kind),
                custom_size: Some(Vec2::splat(TILE_SIZE * 0.78)),
                ..default()
            },
            transform: Transform::from_translation(grid_to_world(grid_position, 0.5)),
            ..default()
        },
        Building { camp, kind },
        MapPosition(grid_position),
    ));
}
