use bevy::{prelude::*, window::PrimaryWindow};

use super::action_runner::run_available_action;
use super::components::{
    Building, ContextMenuActionButton, ContextMenuRoot, ContextMenuText, MapPosition, Unit,
};
use super::grid::{cursor_grid_position, is_inside_map};
use super::interaction::{
    available_action_for_intent, context_menu_lines, AvailableAction, InteractionIntent,
};
use super::resources::{
    AppMeshes, ContextMenu, FactionSelection, GameState, SelectedBuilding, SelectedUnit,
};
use crate::Camp;

const NORMAL_BUTTON: Color = Color::srgb(0.12, 0.14, 0.18);
const HOVERED_BUTTON: Color = Color::srgb(0.2, 0.24, 0.32);
const PRESSED_BUTTON: Color = Color::srgb(0.32, 0.38, 0.5);

#[derive(Resource, Default)]
pub(super) struct PendingContextAction(pub(super) Option<AvailableAction>);

pub(super) fn spawn_context_menu(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::None,
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    row_gap: Val::Px(8.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    max_width: Val::Px(300.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.03, 0.035, 0.045, 0.94).into(),
                ..default()
            },
            ContextMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::srgb(0.98, 0.94, 0.78),
                        ..default()
                    },
                ),
                ContextMenuText,
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            display: Display::None,
                            padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ContextMenuActionButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Executer",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.92, 0.94, 0.86),
                            ..default()
                        },
                    ));
                });
        });
}

pub(super) fn handle_context_menu_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    game: Res<GameState>,
    faction_selection: Res<FactionSelection>,
    selected_unit: Res<SelectedUnit>,
    selected_building: Res<SelectedBuilding>,
    units: Query<(Entity, &MapPosition, &Unit), Without<Building>>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
    mut context_menu: ResMut<ContextMenu>,
    mut pending_action: ResMut<PendingContextAction>,
) {
    if faction_selection.0 || !mouse_buttons.just_pressed(MouseButton::Right) {
        return;
    }

    context_menu.lines.clear();
    pending_action.0 = None;
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(screen_position) = window.cursor_position() else {
        return;
    };
    let Some(clicked_position) = cursor_grid_position(&windows, &cameras) else {
        return;
    };
    if !is_inside_map(clicked_position) {
        return;
    }

    context_menu.screen_position =
        Vec2::new(screen_position.x, window.height() - screen_position.y);
    context_menu.lines = context_menu_lines(
        InteractionIntent::ContextMenu {
            position: clicked_position,
        },
        game.0.faction(Camp::Human),
        &selected_unit,
        &selected_building,
        &units,
        &buildings,
    );
    let action = available_action_for_intent(
        InteractionIntent::PrimaryClick {
            position: clicked_position,
        },
        &selected_unit,
        &selected_building,
        &units,
        &buildings,
    );
    pending_action.0 = match action {
        AvailableAction::None => None,
        _ => Some(action),
    };
}

pub(super) fn update_context_menu(
    context_menu: Res<ContextMenu>,
    pending_action: Res<PendingContextAction>,
    mut roots: Query<&mut Style, With<ContextMenuRoot>>,
    mut texts: Query<&mut Text, With<ContextMenuText>>,
    mut buttons: Query<&mut Style, (With<ContextMenuActionButton>, Without<ContextMenuRoot>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut root_style) = roots.get_single_mut() else {
        return;
    };
    let Ok(mut text) = texts.get_single_mut() else {
        return;
    };
    let Ok(mut button_style) = buttons.get_single_mut() else {
        return;
    };

    if context_menu.lines.is_empty() {
        root_style.display = Display::None;
        return;
    }

    text.sections[0].value = context_menu.lines.join("\n");
    let (left, top) = menu_position(&context_menu, &windows);
    root_style.display = Display::Flex;
    root_style.left = Val::Px(left);
    root_style.top = Val::Px(top);
    button_style.display = if pending_action.0.is_some() {
        Display::Flex
    } else {
        Display::None
    };
}

fn menu_position(
    context_menu: &ContextMenu,
    windows: &Query<&Window, With<PrimaryWindow>>,
) -> (f32, f32) {
    let left = context_menu.screen_position.x + 12.0;
    let top = context_menu.screen_position.y + 12.0;
    let Ok(window) = windows.get_single() else {
        return (left, top);
    };

    (
        left.min(window.width() - 320.0).max(12.0),
        top.min(window.height() - 220.0).max(12.0),
    )
}

pub(super) fn handle_context_menu_action_button(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    app_meshes: Res<AppMeshes>,
    mut game: ResMut<GameState>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut context_menu: ResMut<ContextMenu>,
    mut pending_action: ResMut<PendingContextAction>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ContextMenuActionButton>),
    >,
    mut units: Query<(Entity, &mut MapPosition, &mut Transform, &Unit), Without<Building>>,
    buildings: Query<(Entity, &MapPosition, &Building), Without<Unit>>,
) {
    for (interaction, mut background) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                *background = PRESSED_BUTTON.into();
                let Some(action) = pending_action.0 else {
                    continue;
                };
                run_available_action(
                    action,
                    &mut commands,
                    &app_meshes,
                    &mut materials,
                    &mut game,
                    &mut selected_unit,
                    &mut selected_building,
                    &mut units,
                    &buildings,
                );
                context_menu.lines.clear();
                pending_action.0 = None;
            }
            Interaction::Hovered => *background = HOVERED_BUTTON.into(),
            Interaction::None => *background = NORMAL_BUTTON.into(),
        }
    }
}
