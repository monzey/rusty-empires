use bevy::prelude::*;

use super::components::{Building, MapPosition, Unit};
use super::resources::{AppMeshes, FactionSelection, GameState, SelectedBuilding, SelectedUnit};
use super::selection::clear_selection;
use super::setup::spawn_building;
use super::sync::{apply_game_events, log_resource_events};
use crate::{Action, BuildingKind, Camp, Event, GridPosition};

const NORMAL_BUTTON: Color = Color::srgb(0.12, 0.14, 0.18);
const HOVERED_BUTTON: Color = Color::srgb(0.2, 0.24, 0.32);
const PRESSED_BUTTON: Color = Color::srgb(0.32, 0.38, 0.5);

#[derive(Component, Clone, Copy)]
pub(super) struct ActionBarButton(ActionBarAction);

#[derive(Clone, Copy)]
enum ActionBarAction {
    Build(BuildingKind),
    RecruitSoldier,
    RecruitArcher,
    RecruitUniqueUnit,
    RecruitVillager,
    TradeGoldForFood,
    TradeFoodForGold,
    ResearchAgriculture,
    ResearchMilitaryTraining,
    EndTurn,
}

pub(super) fn spawn_action_bar(commands: &mut Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(372.0),
                right: Val::Px(12.0),
                bottom: Val::Px(12.0),
                padding: UiRect::all(Val::Px(8.0)),
                column_gap: Val::Px(6.0),
                row_gap: Val::Px(6.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            background_color: Color::srgba(0.03, 0.035, 0.045, 0.82).into(),
            ..default()
        })
        .with_children(|parent| {
            spawn_button(
                parent,
                "Mine",
                ActionBarAction::Build(BuildingKind::GoldMine),
            );
            spawn_button(parent, "Ferme", ActionBarAction::Build(BuildingKind::Farm));
            spawn_button(parent, "Forum", ActionBarAction::Build(BuildingKind::Forum));
            spawn_button(
                parent,
                "Caserne",
                ActionBarAction::Build(BuildingKind::Barracks),
            );
            spawn_button(
                parent,
                "Marche",
                ActionBarAction::Build(BuildingKind::Market),
            );
            spawn_button(
                parent,
                "Universite",
                ActionBarAction::Build(BuildingKind::University),
            );
            spawn_button(
                parent,
                "Tour",
                ActionBarAction::Build(BuildingKind::Watchtower),
            );
            spawn_button(parent, "Soldat", ActionBarAction::RecruitSoldier);
            spawn_button(parent, "Archer", ActionBarAction::RecruitArcher);
            spawn_button(parent, "Unique", ActionBarAction::RecruitUniqueUnit);
            spawn_button(parent, "Villageois", ActionBarAction::RecruitVillager);
            spawn_button(parent, "Or->Food", ActionBarAction::TradeGoldForFood);
            spawn_button(parent, "Food->Or", ActionBarAction::TradeFoodForGold);
            spawn_button(parent, "Agri", ActionBarAction::ResearchAgriculture);
            spawn_button(
                parent,
                "Militaire",
                ActionBarAction::ResearchMilitaryTraining,
            );
            spawn_button(parent, "Fin tour", ActionBarAction::EndTurn);
        });
}

fn spawn_button(parent: &mut ChildBuilder, label: &'static str, action: ActionBarAction) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    min_width: Val::Px(54.0),
                    min_height: Val::Px(32.0),
                    padding: UiRect::axes(Val::Px(9.0), Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            ActionBarButton(action),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 13.0,
                    color: Color::srgb(0.92, 0.94, 0.86),
                    ..default()
                },
            ));
        });
}

pub(super) fn handle_action_bar_buttons(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    app_meshes: Res<AppMeshes>,
    faction_selection: Res<FactionSelection>,
    mut game: ResMut<GameState>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut buttons: Query<
        (&Interaction, &ActionBarButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: Query<(Entity, &Building, &MapPosition), Without<Unit>>,
) {
    for (interaction, button, mut background) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                *background = PRESSED_BUTTON.into();
                if !faction_selection.0 {
                    run_action_bar_action(
                        button.0,
                        &mut commands,
                        &app_meshes,
                        &mut materials,
                        &mut game,
                        &mut selected_unit,
                        &mut selected_building,
                        &mut units,
                        &buildings,
                    );
                }
            }
            Interaction::Hovered => *background = HOVERED_BUTTON.into(),
            Interaction::None => *background = NORMAL_BUTTON.into(),
        }
    }
}

fn run_action_bar_action(
    action: ActionBarAction,
    commands: &mut Commands,
    app_meshes: &AppMeshes,
    materials: &mut Assets<ColorMaterial>,
    game: &mut GameState,
    selected_unit: &mut SelectedUnit,
    selected_building: &mut SelectedBuilding,
    units: &mut Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &Building, &MapPosition), Without<Unit>>,
) {
    match action {
        ActionBarAction::Build(kind) => handle_build_action(
            commands,
            app_meshes,
            materials,
            game,
            selected_unit,
            units,
            kind,
        ),
        ActionBarAction::RecruitSoldier => handle_recruit_action(
            commands,
            app_meshes,
            materials,
            game,
            selected_building,
            units,
            buildings,
            BuildingKind::Barracks,
            |building_position| Action::RecruitSoldier { building_position },
            "Selectionne une caserne avant de recruter un soldat.",
            "Selectionne une caserne alliee pour recruter un soldat.",
            "Soldat",
        ),
        ActionBarAction::RecruitArcher => handle_recruit_action(
            commands,
            app_meshes,
            materials,
            game,
            selected_building,
            units,
            buildings,
            BuildingKind::Barracks,
            |building_position| Action::RecruitArcher { building_position },
            "Selectionne une caserne avant de recruter un archer.",
            "Selectionne une caserne alliee pour recruter un archer.",
            "Archer",
        ),
        ActionBarAction::RecruitUniqueUnit => handle_recruit_action(
            commands,
            app_meshes,
            materials,
            game,
            selected_building,
            units,
            buildings,
            BuildingKind::Barracks,
            |building_position| Action::RecruitUniqueUnit { building_position },
            "Selectionne une caserne avant de recruter une unite unique.",
            "Selectionne une caserne alliee pour recruter une unite unique.",
            "Unite unique",
        ),
        ActionBarAction::RecruitVillager => handle_recruit_action(
            commands,
            app_meshes,
            materials,
            game,
            selected_building,
            units,
            buildings,
            BuildingKind::Forum,
            |building_position| Action::RecruitVillager { building_position },
            "Selectionne un forum avant de recruter un villageois.",
            "Selectionne un forum allie pour recruter un villageois.",
            "Villageois",
        ),
        ActionBarAction::TradeGoldForFood => handle_target_building_action(
            game,
            selected_building,
            buildings,
            BuildingKind::Market,
            Action::TradeGoldForFood { amount: 10 },
            "Selectionne un marche avant d'echanger des ressources.",
            "Selectionne un marche allie pour echanger des ressources.",
            "Echange refuse",
        ),
        ActionBarAction::TradeFoodForGold => handle_target_building_action(
            game,
            selected_building,
            buildings,
            BuildingKind::Market,
            Action::TradeFoodForGold { amount: 10 },
            "Selectionne un marche avant d'echanger des ressources.",
            "Selectionne un marche allie pour echanger des ressources.",
            "Echange refuse",
        ),
        ActionBarAction::ResearchAgriculture => handle_target_building_action(
            game,
            selected_building,
            buildings,
            BuildingKind::University,
            Action::ResearchAgriculture,
            "Selectionne une universite avant de rechercher une technologie.",
            "Selectionne une universite alliee pour rechercher une technologie.",
            "Recherche refusee",
        ),
        ActionBarAction::ResearchMilitaryTraining => handle_target_building_action(
            game,
            selected_building,
            buildings,
            BuildingKind::University,
            Action::ResearchMilitaryTraining,
            "Selectionne une universite avant de rechercher une technologie.",
            "Selectionne une universite alliee pour rechercher une technologie.",
            "Recherche refusee",
        ),
        ActionBarAction::EndTurn => handle_end_turn_action(game, selected_unit, selected_building),
    }
}

fn handle_build_action(
    commands: &mut Commands,
    app_meshes: &AppMeshes,
    materials: &mut Assets<ColorMaterial>,
    game: &mut GameState,
    selected_unit: &SelectedUnit,
    units: &Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    kind: BuildingKind,
) {
    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(selected_entity) = selected_unit.0 else {
        info!("Selectionne un villageois avant de construire un batiment.");
        return;
    };
    let Ok((_, _, _, unit)) = units.get(selected_entity) else {
        return;
    };
    let action = build_action(kind, unit.id);

    match game.0.apply(action) {
        Ok(events) => spawn_constructed_buildings(commands, app_meshes, materials, &events),
        Err(error) => info!("Construction refusee: {:?}.", error),
    }
}

fn build_action(kind: BuildingKind, unit_id: crate::UnitId) -> Action {
    match kind {
        BuildingKind::GoldMine => Action::BuildGoldMine { unit_id },
        BuildingKind::Farm => Action::BuildFarm { unit_id },
        BuildingKind::Forum => Action::BuildForum { unit_id },
        BuildingKind::Barracks => Action::BuildBarracks { unit_id },
        BuildingKind::Market => Action::BuildMarket { unit_id },
        BuildingKind::University => Action::BuildUniversity { unit_id },
        BuildingKind::Watchtower => Action::BuildWatchtower { unit_id },
    }
}

fn spawn_constructed_buildings(
    commands: &mut Commands,
    app_meshes: &AppMeshes,
    materials: &mut Assets<ColorMaterial>,
    events: &[Event],
) {
    for event in events {
        if let Event::BuildingConstructed {
            camp,
            kind,
            position,
        } = *event
        {
            spawn_building(
                commands,
                &app_meshes.building,
                materials,
                camp,
                kind,
                position,
            );
            info!("{:?} construit en ({}, {}).", kind, position.x, position.y);
        }
    }
}

fn handle_recruit_action(
    commands: &mut Commands,
    app_meshes: &AppMeshes,
    materials: &mut Assets<ColorMaterial>,
    game: &mut GameState,
    selected_building: &SelectedBuilding,
    units: &mut Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: &Query<(Entity, &Building, &MapPosition), Without<Unit>>,
    expected_kind: BuildingKind,
    action: impl FnOnce(GridPosition) -> Action,
    missing_selection_message: &str,
    wrong_building_message: &str,
    recruited_label: &str,
) {
    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(selected_entity) = selected_building.0 else {
        info!("{}", missing_selection_message);
        return;
    };
    let Ok((_, building, position)) = buildings.get(selected_entity) else {
        return;
    };
    if building.camp != Camp::Human || building.kind != expected_kind {
        info!("{}", wrong_building_message);
        return;
    }

    match game.0.apply(action(position.0)) {
        Ok(events) => {
            apply_game_events(&events, commands, &app_meshes.unit, materials, units);
            for event in events {
                if let Event::UnitRecruited {
                    unit_id, position, ..
                } = event
                {
                    info!(
                        "{} {:?} recrute en ({}, {}).",
                        recruited_label, unit_id, position.x, position.y
                    );
                }
            }
        }
        Err(error) => info!("Recrutement refuse: {:?}.", error),
    }
}

fn handle_target_building_action(
    game: &mut GameState,
    selected_building: &SelectedBuilding,
    buildings: &Query<(Entity, &Building, &MapPosition), Without<Unit>>,
    expected_kind: BuildingKind,
    action: Action,
    missing_selection_message: &str,
    wrong_building_message: &str,
    error_label: &str,
) {
    if game.0.current_turn() != Camp::Human {
        return;
    }

    let Some(selected_entity) = selected_building.0 else {
        info!("{}", missing_selection_message);
        return;
    };
    let Ok((_, building, _)) = buildings.get(selected_entity) else {
        return;
    };
    if building.camp != Camp::Human || building.kind != expected_kind {
        info!("{}", wrong_building_message);
        return;
    }

    match game.0.apply(action) {
        Ok(events) => log_resource_events(&events),
        Err(error) => info!("{}: {:?}.", error_label, error),
    }
}

fn handle_end_turn_action(
    game: &mut GameState,
    selected_unit: &mut SelectedUnit,
    selected_building: &mut SelectedBuilding,
) {
    if game.0.current_turn() != Camp::Human {
        return;
    }

    if let Ok(events) = game.0.apply(Action::EndTurn) {
        clear_selection(selected_unit, selected_building);
        log_resource_events(&events);
        info!("Tour: IA.");
    }
}
