use bevy::prelude::*;

use super::components::TopBarText;
use super::resources::{FactionSelection, GameState};
use crate::Camp;

pub(super) fn update_top_bar(
    game: Res<GameState>,
    faction_selection: Res<FactionSelection>,
    mut top_bar: Query<&mut Text, With<TopBarText>>,
) {
    let Ok(mut text) = top_bar.get_single_mut() else {
        return;
    };

    if faction_selection.0 {
        text.sections[0].value = format!(
            "Choisis ta civilisation: 1 Valdorian | 2 Kharzun | 3 Sylvans | 4 Necrarchs | IA: {:?}",
            game.0.faction(Camp::Ai),
        );
        return;
    }

    text.sections[0].value = format!(
        "Tour: {:?} | Civilisation: {:?} | IA: {:?} | Or: {} | Nourriture: {} | Tech: {}{}",
        game.0.current_turn(),
        game.0.faction(Camp::Human),
        game.0.faction(Camp::Ai),
        game.0.gold(Camp::Human),
        game.0.food(Camp::Human),
        game.0.technology_points(Camp::Human),
        winner_text(&game),
    );
}

fn winner_text(game: &GameState) -> String {
    match game.0.winner() {
        Some(winner) => format!(" | Victoire: {:?}", winner),
        None => String::new(),
    }
}
