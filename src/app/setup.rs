use bevy::prelude::*;

use super::components::{Building, MapPosition, Tile, Unit};
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
            let color = match game.0.natural_resource_at(position) {
                Some(NaturalResource::GoldDeposit) => Color::srgb(0.55, 0.42, 0.12),
                Some(NaturalResource::Field) => Color::srgb(0.42, 0.46, 0.16),
                None if (x + y) % 2 == 0 => Color::srgb(0.22, 0.31, 0.22),
                None => Color::srgb(0.18, 0.27, 0.18),
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

    info!("Boucle initiale: clic sur ton villageois bleu, clic sur une case adjacente libre pour bouger, clic sur une unite ennemie adjacente pour attaquer, B pour construire une mine, F pour construire une ferme, Espace/Entree pour finir le tour.");
}

fn spawn_unit(
    commands: &mut Commands,
    id: UnitId,
    kind: UnitKind,
    camp: Camp,
    grid_position: GridPosition,
) {
    let size = match kind {
        UnitKind::Villager => TILE_SIZE * 0.62,
        UnitKind::Soldier => TILE_SIZE * 0.72,
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
        Building {
            _camp: camp,
            _kind: kind,
        },
        MapPosition(grid_position),
    ));
}
