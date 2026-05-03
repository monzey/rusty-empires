use rusty_empires::{
    Action, BuildingKind, Camp, CombatError, Event, Game, GameError, GridPosition, UnitId, UnitKind,
};

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
    let ai_soldier = game
        .soldier_id(Camp::Ai)
        .expect("AI soldier should exist at game start");

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
            target_id: ai_soldier,
        }),
        Err(GameError::Combat(CombatError::OutOfRange))
    );
}

#[test]
fn human_cannot_attack_enemy_unit_outside_vision() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist at game start");

    assert!(!game.is_visible(
        Camp::Human,
        game.unit_position(ai_villager)
            .expect("AI villager should have a position")
    ));
    assert_eq!(
        game.apply(Action::AttackUnit {
            attacker_id: human_villager,
            target_id: ai_villager,
        }),
        Err(GameError::Combat(CombatError::TargetNotVisible))
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
fn unit_can_move_then_attack_in_the_same_turn() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");
    let ai_soldier = game
        .soldier_id(Camp::Ai)
        .expect("AI soldier should exist at game start");

    game.apply(Action::MoveUnit {
        unit_id: human_soldier,
        to: GridPosition { x: 5, y: 3 },
    })
    .expect("soldier should be able to move before attacking");

    let events = game
        .apply(Action::AttackUnit {
            attacker_id: human_soldier,
            target_id: ai_soldier,
        })
        .expect("soldier should be able to attack after moving into range");

    assert!(events.contains(&Event::UnitDamaged {
        unit_id: ai_soldier,
        amount: 4,
        remaining_health: 6,
    }));
}

#[test]
fn unit_cannot_attack_then_move_in_the_same_turn() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");
    let ai_soldier = game
        .soldier_id(Camp::Ai)
        .expect("AI soldier should exist at game start");

    game.apply(Action::AttackUnit {
        attacker_id: human_soldier,
        target_id: ai_soldier,
    })
    .expect("soldier should be able to attack before moving");

    assert_eq!(
        game.apply(Action::MoveUnit {
            unit_id: human_soldier,
            to: GridPosition { x: 4, y: 3 },
        }),
        Err(GameError::Move(rusty_empires::MoveError::AlreadyActed))
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

#[test]
fn human_can_attack_adjacent_enemy_building() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");
    let forum_position = build_ai_forum_next_to_human_soldier(&mut game);

    let events = game
        .apply(Action::AttackBuilding {
            attacker_id: human_soldier,
            target_position: forum_position,
        })
        .expect("human soldier should be able to attack an adjacent enemy forum");

    assert_eq!(
        events,
        vec![
            Event::BuildingDamaged {
                camp: Camp::Ai,
                kind: BuildingKind::Forum,
                position: forum_position,
                amount: 6,
                remaining_health: 14,
            },
            Event::UnitActed {
                unit_id: human_soldier,
            },
        ]
    );
    assert_eq!(
        game.building_at(forum_position),
        Some((Camp::Ai, BuildingKind::Forum))
    );
}

#[test]
fn human_cannot_attack_enemy_building_outside_vision() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");
    let forum_position = build_far_ai_forum_and_return_to_human(&mut game);

    assert!(!game.is_visible(Camp::Human, forum_position));
    assert_eq!(
        game.apply(Action::AttackBuilding {
            attacker_id: human_soldier,
            target_position: forum_position,
        }),
        Err(GameError::Combat(CombatError::TargetNotVisible))
    );
}

#[test]
fn archer_can_attack_enemy_unit_two_tiles_away() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let archer = recruit_human_archer(&mut game);
    let ai_soldier = move_ai_soldier_and_return_to_human(&mut game, GridPosition { x: 2, y: 4 });

    let events = game
        .apply(Action::AttackUnit {
            attacker_id: archer,
            target_id: ai_soldier,
        })
        .expect("archer should attack an enemy two tiles away");

    assert!(events.contains(&Event::UnitDamaged {
        unit_id: ai_soldier,
        amount: 2,
        remaining_health: 8,
    }));
}

#[test]
fn archer_cannot_attack_enemy_unit_three_tiles_away() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let archer = recruit_human_archer(&mut game);
    let ai_soldier = move_ai_soldier_and_return_to_human(&mut game, GridPosition { x: 3, y: 4 });

    assert_eq!(
        game.apply(Action::AttackUnit {
            attacker_id: archer,
            target_id: ai_soldier,
        }),
        Err(GameError::Combat(CombatError::OutOfRange))
    );
}

#[test]
fn building_is_removed_when_destroyed() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist at game start");
    let forum_position = build_ai_forum_next_to_human_soldier(&mut game);

    for _ in 0..3 {
        game.apply(Action::AttackBuilding {
            attacker_id: human_soldier,
            target_position: forum_position,
        })
        .expect("human soldier should damage the AI forum");
        game.apply(Action::EndTurn)
            .expect("human should be able to end turn after attacking");
        game.apply(Action::RunAiTurn)
            .expect("AI should pass the turn back after acting");
    }

    let events = game
        .apply(Action::AttackBuilding {
            attacker_id: human_soldier,
            target_position: forum_position,
        })
        .expect("human soldier should destroy the AI forum");

    assert!(events.contains(&Event::BuildingDestroyed {
        camp: Camp::Ai,
        kind: BuildingKind::Forum,
        position: forum_position,
    }));
    assert_eq!(game.building_at(forum_position), None);
}

#[test]
fn game_is_won_when_last_enemy_building_is_destroyed() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let forum_position = build_ai_forum_next_to_human_soldier(&mut game);

    let unit_defeat_events = defeat_all_ai_units(&mut game);
    assert!(!unit_defeat_events.contains(&Event::GameWon { camp: Camp::Human }));

    let events = destroy_building_with_human_soldier(&mut game, forum_position);

    assert!(events.contains(&Event::GameWon { camp: Camp::Human }));
    assert_eq!(game.winner(), Some(Camp::Human));
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

fn build_ai_forum_next_to_human_soldier(game: &mut Game) -> GridPosition {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist at game start");
    let forum_position = GridPosition { x: 4, y: 3 };

    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: forum_position,
    })
    .expect("AI villager should be able to move next to the human soldier");
    game.apply(Action::BuildForum {
        unit_id: ai_villager,
    })
    .expect("AI villager should be able to build a forum");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after building a forum");

    forum_position
}

fn build_far_ai_forum_and_return_to_human(game: &mut Game) -> GridPosition {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist at game start");
    let forum_position = GridPosition { x: 8, y: 3 };

    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: forum_position,
    })
    .expect("AI villager should be able to move to a non-resource tile");
    game.apply(Action::BuildForum {
        unit_id: ai_villager,
    })
    .expect("AI villager should be able to build a forum");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after building a forum");

    forum_position
}

fn recruit_human_archer(game: &mut Game) -> UnitId {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    game.apply(Action::BuildGoldMine {
        unit_id: human_villager,
    })
    .expect("villager should be able to build a gold mine");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 2, y: 3 },
    })
    .expect("villager should be able to move onto the field");
    game.apply(Action::BuildFarm {
        unit_id: human_villager,
    })
    .expect("villager should be able to build a farm");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 3 },
    })
    .expect("villager should be able to move to a non-resource tile");
    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("villager should be able to build a forum");
    pass_turn_back_to_human(game);

    let barracks_position = GridPosition { x: 0, y: 4 };
    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: barracks_position,
    })
    .expect("villager should be able to move next to the forum");
    game.apply(Action::BuildBarracks {
        unit_id: human_villager,
    })
    .expect("villager should be able to build a barracks next to the forum");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 3 },
    })
    .expect("villager should be able to move away from the barracks");

    game.apply(Action::RecruitArcher {
        building_position: barracks_position,
    })
    .expect("archer should be recruitable");

    let archer = rusty_empires::UnitId(5);
    assert_eq!(game.unit_kind(archer), Some(UnitKind::Archer));
    archer
}

fn move_ai_soldier_and_return_to_human(game: &mut Game, to: GridPosition) -> UnitId {
    let ai_soldier = game.soldier_id(Camp::Ai).expect("AI soldier should exist");

    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::MoveUnit {
        unit_id: ai_soldier,
        to,
    })
    .expect("AI soldier should be able to move into archer range test position");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after moving soldier");

    ai_soldier
}

fn pass_turn_back_to_human(game: &mut Game) {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");
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

fn destroy_building_with_human_soldier(game: &mut Game, position: GridPosition) -> Vec<Event> {
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist");

    for _ in 0..3 {
        game.apply(Action::AttackBuilding {
            attacker_id: human_soldier,
            target_position: position,
        })
        .expect("human soldier should damage the building");
        game.apply(Action::EndTurn)
            .expect("human should be able to end turn after attacking");
        game.apply(Action::RunAiTurn)
            .expect("AI should pass the turn back without units");
    }

    game.apply(Action::AttackBuilding {
        attacker_id: human_soldier,
        target_position: position,
    })
    .expect("human soldier should destroy the building")
}

fn grid_distance(a: GridPosition, b: GridPosition) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
