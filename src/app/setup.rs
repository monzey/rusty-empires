use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

use super::action_bar::spawn_action_bar;
use super::components::{Building, MapPosition, SelectionPanelText, TopBarText, Unit};
use super::constants::{TILE_HEIGHT, TILE_SIZE, TILE_WIDTH};
use super::context_menu::spawn_context_menu;
use super::grid::grid_to_world;
use super::resources::{AppMeshes, GameState};
use super::tooltip::spawn_tooltip;
use super::visuals::{building_color, unit_color};
use crate::{BuildingKind, Camp, GridPosition, UnitId, UnitKind};

pub(super) fn setup(
    mut commands: Commands,
    game: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let human_start = game
        .0
        .villager_position(Camp::Human)
        .expect("human villager should exist at game start");
    let start_world = grid_to_world(human_start, 0.0);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(start_world.x, start_world.y, 999.9),
        ..default()
    });
    let tile_mesh = meshes.add(tile_mesh());
    let unit_mesh = meshes.add(unit_mesh());
    let building_mesh = meshes.add(building_mesh());
    commands.insert_resource(AppMeshes {
        tile: tile_mesh.clone(),
        unit: unit_mesh.clone(),
        building: building_mesh.clone(),
    });

    spawn_unit(
        &mut commands,
        &unit_mesh,
        &mut materials,
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
        &unit_mesh,
        &mut materials,
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
        &unit_mesh,
        &mut materials,
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
        &unit_mesh,
        &mut materials,
        game.0
            .soldier_id(Camp::Ai)
            .expect("AI soldier should have an id at game start"),
        UnitKind::Soldier,
        Camp::Ai,
        game.0
            .soldier_position(Camp::Ai)
            .expect("AI soldier should exist at game start"),
    );

    spawn_top_bar(&mut commands);
    spawn_selection_panel(&mut commands);
    spawn_action_bar(&mut commands);
    spawn_context_menu(&mut commands);
    spawn_tooltip(&mut commands);

    info!("Choisis ta civilisation: 1 Valdorian, 2 Kharzun, 3 Sylvans, 4 Necrarchs.");
    info!("Apres choix: clic unite selection, clic droit menu contextuel, Echap deselection, clic case libre bouger, clic ennemi attaquer, B mine, F ferme, T forum, R caserne, M marche, U universite, O tour de guet, S soldat, A archer, C unite unique, V villageois, G/N commerce, H Agriculture, Y entrainement militaire, fleches camera, molette zoom, Espace/Entree finir tour.");
}

fn tile_mesh() -> Mesh {
    let half_width = (TILE_WIDTH - 2.0) / 2.0;
    let half_height = (TILE_HEIGHT - 2.0) / 2.0;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, half_height, 0.0],
            [half_width, 0.0, 0.0],
            [0.0, -half_height, 0.0],
            [-half_width, 0.0, 0.0],
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.5, 1.0], [1.0, 0.5], [0.5, 0.0], [0.0, 0.5]],
    );
    mesh.insert_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));
    mesh
}

fn unit_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.42, 0.0],
            [0.34, 0.18, 0.0],
            [0.42, -0.08, 0.0],
            [0.18, -0.32, 0.0],
            [0.0, -0.38, 0.0],
            [-0.18, -0.32, 0.0],
            [-0.42, -0.08, 0.0],
            [-0.34, 0.18, 0.0],
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.5, 1.0],
            [0.9, 0.78],
            [1.0, 0.42],
            [0.7, 0.12],
            [0.5, 0.0],
            [0.3, 0.12],
            [0.0, 0.42],
            [0.1, 0.78],
        ],
    );
    mesh.insert_indices(Indices::U32(vec![
        0, 1, 7, 1, 2, 6, 1, 6, 7, 2, 3, 5, 2, 5, 6, 3, 4, 5,
    ]));
    mesh
}

fn building_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.58, 0.0],
            [0.46, 0.32, 0.0],
            [0.46, -0.18, 0.0],
            [0.0, -0.44, 0.0],
            [-0.46, -0.18, 0.0],
            [-0.46, 0.32, 0.0],
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.5, 1.0],
            [1.0, 0.76],
            [1.0, 0.28],
            [0.5, 0.0],
            [0.0, 0.28],
            [0.0, 0.76],
        ],
    );
    mesh.insert_indices(Indices::U32(vec![0, 1, 5, 1, 2, 4, 1, 4, 5, 2, 3, 4]));
    mesh
}

fn spawn_top_bar(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.92, 0.94, 0.86),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            right: Val::Px(12.0),
            top: Val::Px(12.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_background_color(Color::srgba(0.03, 0.035, 0.045, 0.82)),
        TopBarText,
    ));
}

fn spawn_selection_panel(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.92, 0.94, 0.86),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            bottom: Val::Px(12.0),
            width: Val::Px(336.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_background_color(Color::srgba(0.03, 0.035, 0.045, 0.82)),
        SelectionPanelText,
    ));
}

pub(super) fn spawn_unit(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    id: UnitId,
    kind: UnitKind,
    camp: Camp,
    grid_position: GridPosition,
) {
    let size = match kind {
        UnitKind::Villager => TILE_SIZE * 0.46,
        UnitKind::Soldier => TILE_SIZE * 0.54,
        UnitKind::Archer => TILE_SIZE * 0.5,
        UnitKind::ValdorianLegionary => TILE_SIZE * 0.56,
        UnitKind::KharzunShieldbreaker => TILE_SIZE * 0.58,
        UnitKind::ElyrPathfinder => TILE_SIZE * 0.44,
        UnitKind::ObsidianBoneServant => TILE_SIZE * 0.42,
    };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: materials.add(unit_color(camp, kind, false, false)),
            transform: Transform::from_translation(grid_to_world(grid_position, 1.0))
                .with_scale(Vec3::splat(size)),
            ..default()
        },
        Unit { id, camp, kind },
        MapPosition(grid_position),
    ));
}

pub(super) fn spawn_building(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    camp: Camp,
    kind: BuildingKind,
    grid_position: GridPosition,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: materials.add(building_color(camp, kind)),
            transform: Transform::from_translation(grid_to_world(grid_position, 0.5))
                .with_scale(Vec3::splat(TILE_SIZE * 0.58)),
            ..default()
        },
        Building { camp, kind },
        MapPosition(grid_position),
    ));
}
