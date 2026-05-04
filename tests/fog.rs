use rusty_empires::{Action, BuildingKind, Camp, Game, GridPosition};

#[test]
fn initial_visibility_marks_nearby_tiles_visible_and_far_tiles_unexplored() {
    let game = Game::new_single_player_vs_ai(10, 8);

    assert!(game.is_visible(Camp::Human, GridPosition { x: 1, y: 3 }));
    assert!(game.is_explored(Camp::Human, GridPosition { x: 1, y: 3 }));
    assert!(!game.is_visible(Camp::Human, GridPosition { x: 9, y: 7 }));
    assert!(!game.is_explored(Camp::Human, GridPosition { x: 9, y: 7 }));
}

#[test]
fn moving_unit_reveals_new_tiles_and_keeps_old_tiles_explored() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");
    let old_tile = GridPosition { x: 0, y: 1 };
    let new_tile = GridPosition { x: 8, y: 3 };

    assert!(game.is_visible(Camp::Human, old_tile));
    assert!(!game.is_visible(Camp::Human, new_tile));

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 6, y: 3 },
    })
    .expect("villager should be able to move and reveal new tiles");

    assert!(game.is_visible(Camp::Human, new_tile));
    assert!(!game.is_visible(Camp::Human, old_tile));
    assert!(game.is_explored(Camp::Human, old_tile));
}

#[test]
fn building_adds_visibility_for_own_camp() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");
    let revealed_by_forum = GridPosition { x: 0, y: 7 };

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 3 },
    })
    .expect("villager should be able to move to a non-resource tile");
    assert!(!game.is_visible(Camp::Human, revealed_by_forum));

    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("villager should be able to build a forum");

    assert_eq!(
        game.building_at(GridPosition { x: 0, y: 3 }),
        Some((Camp::Human, BuildingKind::Forum))
    );
    assert!(game.is_visible(Camp::Human, revealed_by_forum));
    assert!(game.is_explored(Camp::Human, revealed_by_forum));
}

#[test]
fn watchtower_reveals_tiles_six_tiles_away() {
    let mut game = Game::new_single_player_vs_ai(12, 12);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");
    let revealed_by_watchtower = GridPosition { x: 0, y: 11 };

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 3 },
    })
    .expect("villager should be able to move to a forum site");
    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("villager should be able to build a forum");
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 5 },
    })
    .expect("villager should be able to move to a watchtower cross tile");
    assert!(!game.is_visible(Camp::Human, revealed_by_watchtower));

    game.apply(Action::BuildWatchtower {
        unit_id: human_villager,
    })
    .expect("watchtower should be buildable two orthogonal tiles from the forum");

    assert!(game.is_visible(Camp::Human, revealed_by_watchtower));
    assert!(game.is_explored(Camp::Human, revealed_by_watchtower));
}
