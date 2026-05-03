use rusty_empires::{Action, Camp, Event, Game, GameError, GridPosition, MoveError};

#[test]
fn human_can_move_villager_up_to_five_tiles_during_their_turn() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    assert_eq!(game.current_turn(), Camp::Human);

    let events = game
        .apply(Action::MoveUnit {
            unit_id: human_villager,
            to: GridPosition { x: 6, y: 3 },
        })
        .expect("human villager should be able to move five tiles right");

    assert_eq!(
        events,
        vec![
            Event::UnitMoved {
                unit_id: human_villager,
                from: GridPosition { x: 1, y: 3 },
                to: GridPosition { x: 6, y: 3 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.villager_position(Camp::Human),
        Some(GridPosition { x: 6, y: 3 })
    );
}

#[test]
fn human_villager_cannot_move_to_invalid_tiles_or_act_twice() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    assert_eq!(
        game.apply(Action::MoveUnit {
            unit_id: human_villager,
            to: GridPosition { x: -1, y: 3 }
        }),
        Err(GameError::Move(MoveError::OutsideMap))
    );
    assert_eq!(
        game.villager_position(Camp::Human),
        Some(GridPosition { x: 1, y: 3 })
    );

    assert_eq!(
        game.apply(Action::MoveUnit {
            unit_id: human_villager,
            to: GridPosition { x: 7, y: 3 }
        }),
        Err(GameError::Move(MoveError::OutOfRange))
    );
    assert_eq!(
        game.villager_position(Camp::Human),
        Some(GridPosition { x: 1, y: 3 })
    );

    assert_eq!(
        game.apply(Action::MoveUnit {
            unit_id: human_villager,
            to: GridPosition { x: 8, y: 4 }
        }),
        Err(GameError::Move(MoveError::Occupied))
    );
    assert_eq!(
        game.villager_position(Camp::Human),
        Some(GridPosition { x: 1, y: 3 })
    );

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 2, y: 3 },
    })
    .expect("first valid move should consume the villager action");
    assert_eq!(
        game.apply(Action::MoveUnit {
            unit_id: human_villager,
            to: GridPosition { x: 3, y: 3 }
        }),
        Err(GameError::Move(MoveError::AlreadyActed))
    );
    assert_eq!(
        game.villager_position(Camp::Human),
        Some(GridPosition { x: 2, y: 3 })
    );
}

#[test]
fn human_soldier_can_move_three_tiles_but_not_four() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");

    assert_eq!(
        game.apply(Action::MoveUnit {
            unit_id: human_soldier,
            to: GridPosition { x: 4, y: 1 }
        })
        .expect("human soldier should be able to move three tiles"),
        vec![
            Event::UnitMoved {
                unit_id: human_soldier,
                from: GridPosition { x: 4, y: 4 },
                to: GridPosition { x: 4, y: 1 }
            },
            Event::UnitActed {
                unit_id: human_soldier
            },
        ]
    );

    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");

    assert_eq!(
        game.apply(Action::MoveUnit {
            unit_id: human_soldier,
            to: GridPosition { x: 4, y: 0 }
        }),
        Err(GameError::Move(MoveError::OutOfRange))
    );
}
