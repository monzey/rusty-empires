use rusty_empires::{Action, Camp, CombatError, Event, Game, GameError, GridPosition, UnitId};

#[test]
fn human_soldier_uses_soldier_combat_stats() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");
    let ai_soldier = game
        .soldier_id(Camp::Ai)
        .expect("AI soldier should exist at game start");

    let events = game
        .apply(Action::AttackUnit {
            attacker_id: human_soldier,
            target_id: ai_soldier,
        })
        .expect("initial soldiers should start adjacent and be able to fight");

    assert_eq!(
        events,
        vec![
            Event::UnitDamaged {
                unit_id: ai_soldier,
                amount: 4,
                remaining_health: 6,
            },
            Event::UnitActed {
                unit_id: human_soldier,
            },
        ]
    );
    assert_eq!(game.unit_health(ai_soldier), Some(6));
}

#[test]
fn human_can_attack_adjacent_enemy_unit() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let ai_villager = move_until_ai_is_adjacent_to_human(&mut game);

    let events = game
        .apply(Action::AttackUnit {
            attacker_id: human_villager,
            target_id: ai_villager,
        })
        .expect("human villager should be able to attack an adjacent enemy");

    assert_eq!(
        events,
        vec![
            Event::UnitDamaged {
                unit_id: ai_villager,
                amount: 3,
                remaining_health: 3,
            },
            Event::UnitActed {
                unit_id: human_villager,
            },
        ]
    );
    assert_eq!(game.unit_health(ai_villager), Some(3));
    assert_eq!(game.unit_has_acted(human_villager), Some(true));
}

#[test]
fn human_cannot_attack_invalid_or_out_of_range_targets() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist at game start");

    assert_eq!(
        game.apply(Action::AttackUnit {
            attacker_id: UnitId(999),
            target_id: ai_villager,
        }),
        Err(GameError::Combat(CombatError::NoAttacker))
    );
    assert_eq!(
        game.apply(Action::AttackUnit {
            attacker_id: human_villager,
            target_id: UnitId(999),
        }),
        Err(GameError::Combat(CombatError::NoTarget))
    );
    assert_eq!(
        game.apply(Action::AttackUnit {
            attacker_id: human_villager,
            target_id: human_villager,
        }),
        Err(GameError::Combat(CombatError::FriendlyTarget))
    );
    assert_eq!(
        game.apply(Action::AttackUnit {
            attacker_id: human_villager,
            target_id: ai_villager,
        }),
        Err(GameError::Combat(CombatError::OutOfRange))
    );
}

#[test]
fn attacker_cannot_attack_twice_in_the_same_turn() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let ai_villager = move_until_ai_is_adjacent_to_human(&mut game);

    game.apply(Action::AttackUnit {
        attacker_id: human_villager,
        target_id: ai_villager,
    })
    .expect("first attack should be valid");

    assert_eq!(
        game.apply(Action::AttackUnit {
            attacker_id: human_villager,
            target_id: ai_villager,
        }),
        Err(GameError::Combat(CombatError::AlreadyActed))
    );
}

#[test]
fn unit_is_removed_when_defeated() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let ai_villager = move_until_ai_is_adjacent_to_human(&mut game);

    game.apply(Action::AttackUnit {
        attacker_id: human_villager,
        target_id: ai_villager,
    })
    .expect("first attack should damage the AI villager");
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn after attacking");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after acting");

    let events = game
        .apply(Action::AttackUnit {
            attacker_id: human_villager,
            target_id: ai_villager,
        })
        .expect("second attack should defeat the AI villager");

    assert_eq!(
        events,
        vec![
            Event::UnitDamaged {
                unit_id: ai_villager,
                amount: 3,
                remaining_health: 0,
            },
            Event::UnitDefeated {
                unit_id: ai_villager,
            },
            Event::UnitActed {
                unit_id: human_villager,
            },
        ]
    );
    assert_eq!(game.unit_position(ai_villager), None);
    assert_eq!(game.unit_health(ai_villager), None);
}

#[test]
fn game_is_won_when_last_enemy_unit_is_defeated() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let events = defeat_all_ai_units(&mut game);

    assert!(events.contains(&Event::GameWon { camp: Camp::Human }));
    assert_eq!(game.winner(), Some(Camp::Human));
}

#[test]
fn actions_are_rejected_after_game_is_won() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    defeat_all_ai_units(&mut game);

    assert_eq!(
        game.apply(Action::EndTurn),
        Err(GameError::GameOver {
            winner: Camp::Human
        })
    );
}

fn move_until_ai_is_adjacent_to_human(game: &mut Game) -> UnitId {
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist at game start");

    for _ in 0..10 {
        let human_position = game
            .villager_position(Camp::Human)
            .expect("human villager should still be alive");
        let ai_position = game
            .villager_position(Camp::Ai)
            .expect("AI villager should still be alive");

        if grid_distance(human_position, ai_position) == 1 {
            return ai_villager;
        }

        game.apply(Action::EndTurn)
            .expect("human should be able to end turn while waiting for AI");
        game.apply(Action::RunAiTurn)
            .expect("AI should move toward the human villager");
    }

    panic!("AI villager should become adjacent to the human villager");
}

fn defeat_ai_soldier(game: &mut Game, ai_soldier: UnitId) {
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");

    for _ in 0..2 {
        game.apply(Action::AttackUnit {
            attacker_id: human_soldier,
            target_id: ai_soldier,
        })
        .expect("human soldier should damage adjacent AI soldier");
        game.apply(Action::EndTurn)
            .expect("human should be able to end turn after attacking");
        game.apply(Action::RunAiTurn)
            .expect("AI should pass the turn back after acting");
    }

    game.apply(Action::AttackUnit {
        attacker_id: human_soldier,
        target_id: ai_soldier,
    })
    .expect("human soldier should defeat adjacent AI soldier");
}

fn defeat_all_ai_units(game: &mut Game) -> Vec<Event> {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let ai_soldier = game
        .soldier_id(Camp::Ai)
        .expect("AI soldier should exist at game start");

    defeat_ai_soldier(game, ai_soldier);
    let ai_villager = move_until_ai_is_adjacent_to_human(game);

    game.apply(Action::AttackUnit {
        attacker_id: human_villager,
        target_id: ai_villager,
    })
    .expect("first attack should damage the last AI unit");
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn after attacking");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after acting");

    game.apply(Action::AttackUnit {
        attacker_id: human_villager,
        target_id: ai_villager,
    })
    .expect("second attack should defeat the last AI unit")
}

fn grid_distance(a: GridPosition, b: GridPosition) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
