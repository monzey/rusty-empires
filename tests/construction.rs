use rusty_empires::{Action, BuildError, BuildingKind, Camp, Event, Game, GameError, GridPosition};

#[test]
fn human_can_build_gold_mine_only_when_standing_on_gold_deposit() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    let events = game
        .apply(Action::BuildGoldMine {
            unit_id: human_villager,
        })
        .expect("human villager starts on a gold deposit and can build a gold mine");

    assert_eq!(
        events,
        vec![
            Event::BuildingConstructed {
                camp: Camp::Human,
                kind: BuildingKind::GoldMine,
                position: GridPosition { x: 1, y: 3 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.building_at(GridPosition { x: 1, y: 3 }),
        Some((Camp::Human, BuildingKind::GoldMine))
    );
    assert_eq!(game.unit_has_acted(human_villager), Some(true));
}

#[test]
fn human_cannot_build_gold_mine_away_from_gold_deposit() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 2, y: 3 },
    })
    .expect("moving away from the initial gold deposit should be valid");
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should be able to pass the turn back");

    assert_eq!(
        game.apply(Action::BuildGoldMine {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NoGoldDeposit))
    );
    assert_eq!(game.building_at(GridPosition { x: 2, y: 3 }), None);
}

#[test]
fn human_can_build_farm_only_when_standing_on_field() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 2, y: 3 },
    })
    .expect("human villager should be able to move onto the adjacent field");
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should be able to pass the turn back");

    let events = game
        .apply(Action::BuildFarm {
            unit_id: human_villager,
        })
        .expect("human villager standing on a field can build a farm");

    assert_eq!(
        events,
        vec![
            Event::BuildingConstructed {
                camp: Camp::Human,
                kind: BuildingKind::Farm,
                position: GridPosition { x: 2, y: 3 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.building_at(GridPosition { x: 2, y: 3 }),
        Some((Camp::Human, BuildingKind::Farm))
    );
    assert_eq!(game.unit_has_acted(human_villager), Some(true));
}

#[test]
fn human_cannot_build_farm_away_from_field() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    assert_eq!(
        game.apply(Action::BuildFarm {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NoField))
    );
    assert_eq!(game.building_at(GridPosition { x: 1, y: 3 }), None);
}

#[test]
fn soldier_cannot_build() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");

    assert_eq!(
        game.apply(Action::BuildFarm {
            unit_id: human_soldier,
        }),
        Err(GameError::Build(BuildError::NotBuilder))
    );
}
