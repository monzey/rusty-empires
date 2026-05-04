use bevy::prelude::*;

use super::resources::{ContextMenu, FactionSelection, SelectedBuilding, SelectedUnit};

pub(super) fn clear_selection(
    selected_unit: &mut SelectedUnit,
    selected_building: &mut SelectedBuilding,
) {
    selected_unit.0 = None;
    selected_building.0 = None;
}

pub(super) fn handle_deselect_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    faction_selection: Res<FactionSelection>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut context_menu: ResMut<ContextMenu>,
) {
    if faction_selection.0 || !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    context_menu.lines.clear();
    if selected_unit.0.is_some() || selected_building.0.is_some() {
        clear_selection(&mut selected_unit, &mut selected_building);
        info!("Selection annulee.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_selection_removes_selected_unit_and_building() {
        let mut selected_unit = SelectedUnit(Some(Entity::from_raw(1)));
        let mut selected_building = SelectedBuilding(Some(Entity::from_raw(2)));

        clear_selection(&mut selected_unit, &mut selected_building);

        assert_eq!(selected_unit.0, None);
        assert_eq!(selected_building.0, None);
    }
}
