use rusty_empires::{Action, BuildError, BuildingKind, Camp, Event, Game, GameError, GridPosition};

#[test]
fn human_can_build_gold_mine_only_when_standing_on_gold_deposit() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    let gold_before = game.gold(Camp::Human);
    let food_before = game.food(Camp::Human);

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
    assert_eq!(game.gold(Camp::Human), gold_before - 60);
    assert_eq!(game.food(Camp::Human), food_before);
}

#[test]
fn construction_requires_enough_resources() {
    let mut game = Game::new_single_player_vs_ai_with_resources(10, 8, 59, 500, 1000, 500);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    assert_eq!(
        game.apply(Action::BuildGoldMine {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NotEnoughGold))
    );
    assert_eq!(game.gold(Camp::Human), 59);
    assert_eq!(game.building_at(GridPosition { x: 1, y: 3 }), None);

    let mut game = Game::new_single_player_vs_ai_with_resources(10, 8, 250, 99, 1000, 500);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");
    move_human_villager_and_restore_turn(&mut game, GridPosition { x: 0, y: 3 });

    assert_eq!(
        game.apply(Action::BuildForum {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NotEnoughFood))
    );
    assert_eq!(game.food(Camp::Human), 99);
    assert_eq!(game.building_at(GridPosition { x: 0, y: 3 }), None);
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
fn human_can_move_then_build_farm_when_standing_on_field() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 2, y: 3 },
    })
    .expect("human villager should be able to move onto the adjacent field");

    let events = game
        .apply(Action::BuildFarm {
            unit_id: human_villager,
        })
        .expect("human villager can build a farm after moving onto a field");

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

#[test]
fn villager_can_build_forum_anywhere_except_on_natural_resource() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    assert_eq!(
        game.apply(Action::BuildForum {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NaturalResourcePresent))
    );

    move_human_villager_and_restore_turn(&mut game, GridPosition { x: 0, y: 3 });

    let events = game
        .apply(Action::BuildForum {
            unit_id: human_villager,
        })
        .expect("villager should be able to build a forum on a non-resource tile");

    assert_eq!(
        events,
        vec![
            Event::BuildingConstructed {
                camp: Camp::Human,
                kind: BuildingKind::Forum,
                position: GridPosition { x: 0, y: 3 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.building_at(GridPosition { x: 0, y: 3 }),
        Some((Camp::Human, BuildingKind::Forum))
    );
}

#[test]
fn barracks_must_be_built_adjacent_to_own_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    move_human_villager_and_restore_turn(&mut game, GridPosition { x: 0, y: 3 });

    assert_eq!(
        game.apply(Action::BuildBarracks {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NoAdjacentForum))
    );

    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("forum should be buildable before building a barracks");
    pass_turn_back_to_human(&mut game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 4 },
    })
    .expect("villager should be able to move adjacent to the forum");
    pass_turn_back_to_human(&mut game);

    let events = game
        .apply(Action::BuildBarracks {
            unit_id: human_villager,
        })
        .expect("barracks should be buildable next to an allied forum");

    assert_eq!(
        events,
        vec![
            Event::BuildingConstructed {
                camp: Camp::Human,
                kind: BuildingKind::Barracks,
                position: GridPosition { x: 0, y: 4 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.building_at(GridPosition { x: 0, y: 4 }),
        Some((Camp::Human, BuildingKind::Barracks))
    );
}

#[test]
fn market_must_be_built_adjacent_to_own_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    move_human_villager_and_restore_turn(&mut game, GridPosition { x: 0, y: 3 });

    assert_eq!(
        game.apply(Action::BuildMarket {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NoAdjacentForum))
    );

    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("forum should be buildable before building a market");
    pass_turn_back_to_human(&mut game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 4 },
    })
    .expect("villager should be able to move adjacent to the forum");
    pass_turn_back_to_human(&mut game);

    let events = game
        .apply(Action::BuildMarket {
            unit_id: human_villager,
        })
        .expect("market should be buildable next to an allied forum");

    assert_eq!(
        events,
        vec![
            Event::BuildingConstructed {
                camp: Camp::Human,
                kind: BuildingKind::Market,
                position: GridPosition { x: 0, y: 4 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.building_at(GridPosition { x: 0, y: 4 }),
        Some((Camp::Human, BuildingKind::Market))
    );
}

#[test]
fn university_must_be_built_adjacent_to_own_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    move_human_villager_and_restore_turn(&mut game, GridPosition { x: 0, y: 3 });

    assert_eq!(
        game.apply(Action::BuildUniversity {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NoAdjacentForum))
    );

    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("forum should be buildable before building a university");
    pass_turn_back_to_human(&mut game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 4 },
    })
    .expect("villager should be able to move adjacent to the forum");
    pass_turn_back_to_human(&mut game);

    let events = game
        .apply(Action::BuildUniversity {
            unit_id: human_villager,
        })
        .expect("university should be buildable next to an allied forum");

    assert_eq!(
        events,
        vec![
            Event::BuildingConstructed {
                camp: Camp::Human,
                kind: BuildingKind::University,
                position: GridPosition { x: 0, y: 4 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.building_at(GridPosition { x: 0, y: 4 }),
        Some((Camp::Human, BuildingKind::University))
    );
}

#[test]
fn watchtower_must_be_built_in_a_cross_two_tiles_from_own_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    move_human_villager_and_restore_turn(&mut game, GridPosition { x: 0, y: 3 });

    assert_eq!(
        game.apply(Action::BuildWatchtower {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NoWatchtowerCross))
    );

    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("forum should be buildable before building a watchtower");
    pass_turn_back_to_human(&mut game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 4 },
    })
    .expect("villager should be able to move adjacent to the forum");
    pass_turn_back_to_human(&mut game);
    assert_eq!(
        game.apply(Action::BuildWatchtower {
            unit_id: human_villager,
        }),
        Err(GameError::Build(BuildError::NoWatchtowerCross))
    );

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 5 },
    })
    .expect("villager should be able to move two tiles from the forum");
    pass_turn_back_to_human(&mut game);

    let events = game
        .apply(Action::BuildWatchtower {
            unit_id: human_villager,
        })
        .expect("watchtower should be buildable two orthogonal tiles from an allied forum");

    assert_eq!(
        events,
        vec![
            Event::BuildingConstructed {
                camp: Camp::Human,
                kind: BuildingKind::Watchtower,
                position: GridPosition { x: 0, y: 5 }
            },
            Event::UnitActed {
                unit_id: human_villager
            },
        ]
    );
    assert_eq!(
        game.building_at(GridPosition { x: 0, y: 5 }),
        Some((Camp::Human, BuildingKind::Watchtower))
    );
}

fn move_human_villager_and_restore_turn(game: &mut Game, to: GridPosition) {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to,
    })
    .expect("human villager move should be valid");
    pass_turn_back_to_human(game);
}

fn pass_turn_back_to_human(game: &mut Game) {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");
}
