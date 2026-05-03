use rusty_empires::{Camp, Game, GridPosition, UnitKind};

#[test]
fn initial_game_starts_with_villagers_and_soldiers() {
    let game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist at game start");
    let ai_soldier = game
        .soldier_id(Camp::Ai)
        .expect("AI soldier should exist at game start");

    assert_eq!(game.unit_kind(human_villager), Some(UnitKind::Villager));
    assert_eq!(game.unit_kind(human_soldier), Some(UnitKind::Soldier));
    assert_eq!(game.unit_kind(ai_villager), Some(UnitKind::Villager));
    assert_eq!(game.unit_kind(ai_soldier), Some(UnitKind::Soldier));
    assert_eq!(
        game.soldier_position(Camp::Human),
        Some(GridPosition { x: 4, y: 4 })
    );
    assert_eq!(
        game.soldier_position(Camp::Ai),
        Some(GridPosition { x: 5, y: 4 })
    );
}

#[test]
fn soldiers_have_more_health_than_villagers() {
    let game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");

    assert_eq!(game.unit_health(human_villager), Some(6));
    assert_eq!(game.unit_health(human_soldier), Some(10));
}
