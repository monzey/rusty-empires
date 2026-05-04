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

#[test]
fn large_map_initial_state_keeps_units_and_resources_inside_map() {
    let game = Game::new_single_player_vs_ai(24, 16);

    for camp in [Camp::Human, Camp::Ai] {
        let villager_position = game
            .villager_position(camp)
            .expect("camp villager should exist");
        let soldier_position = game
            .soldier_position(camp)
            .expect("camp soldier should exist");

        assert!(is_inside(villager_position, 24, 16));
        assert!(is_inside(soldier_position, 24, 16));
    }

    assert_eq!(
        game.natural_resource_at(GridPosition { x: 1, y: 3 }),
        Some(rusty_empires::NaturalResource::GoldDeposit)
    );
    assert_eq!(
        game.natural_resource_at(GridPosition { x: 22, y: 12 }),
        Some(rusty_empires::NaturalResource::GoldDeposit)
    );
    assert!(!game.is_visible(Camp::Human, GridPosition { x: 23, y: 15 }));
    assert!(!game.is_explored(Camp::Human, GridPosition { x: 23, y: 15 }));
}

fn is_inside(position: GridPosition, width: i32, height: i32) -> bool {
    position.x >= 0 && position.x < width && position.y >= 0 && position.y < height
}
