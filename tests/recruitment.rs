use rusty_empires::{
    Action, BuildingKind, Camp, Event, Game, GameError, GridPosition, RecruitError, UnitKind,
};

#[test]
fn soldier_can_be_recruited_on_empty_allied_barracks() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let barracks_position = prepare_empty_human_barracks(&mut game, true);
    let food_before = game.food(Camp::Human);

    let events = game
        .apply(Action::RecruitSoldier {
            building_position: barracks_position,
        })
        .expect("soldier should be recruitable on an empty allied barracks");

    assert_eq!(
        events,
        vec![Event::UnitRecruited {
            unit_id: rusty_empires::UnitId(5),
            camp: Camp::Human,
            kind: UnitKind::Soldier,
            position: barracks_position,
        }]
    );
    assert_eq!(game.food(Camp::Human), food_before - 10);
    assert_eq!(
        game.unit_position(rusty_empires::UnitId(5)),
        Some(barracks_position)
    );
    assert_eq!(
        game.unit_kind(rusty_empires::UnitId(5)),
        Some(UnitKind::Soldier)
    );
}

#[test]
fn soldier_cannot_be_recruited_without_food() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let barracks_position = prepare_empty_human_barracks(&mut game, false);

    assert_eq!(
        game.apply(Action::RecruitSoldier {
            building_position: barracks_position,
        }),
        Err(GameError::Recruit(RecruitError::NotEnoughFood))
    );
}

#[test]
fn soldier_cannot_be_recruited_on_occupied_barracks() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let barracks_position = prepare_occupied_human_barracks(&mut game, true);

    assert_eq!(
        game.apply(Action::RecruitSoldier {
            building_position: barracks_position,
        }),
        Err(GameError::Recruit(RecruitError::Occupied))
    );
}

#[test]
fn soldier_cannot_be_recruited_from_missing_or_enemy_barracks() {
    let mut game = Game::new_single_player_vs_ai(10, 8);

    assert_eq!(
        game.apply(Action::RecruitSoldier {
            building_position: GridPosition { x: 0, y: 0 },
        }),
        Err(GameError::Recruit(RecruitError::NoBarracks))
    );

    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist at game start");
    game.apply(Action::BuildGoldMine {
        unit_id: ai_villager,
    })
    .expect("AI should be able to build a non-barracks building");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");

    assert_eq!(
        game.apply(Action::RecruitSoldier {
            building_position: GridPosition { x: 8, y: 4 },
        }),
        Err(GameError::Recruit(RecruitError::NoBarracks))
    );

    let enemy_barracks_position = prepare_enemy_barracks(&mut game);
    assert_eq!(
        game.apply(Action::RecruitSoldier {
            building_position: enemy_barracks_position,
        }),
        Err(GameError::Recruit(RecruitError::NotOwnerTurn))
    );
}

#[test]
fn villager_can_be_recruited_on_empty_allied_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let forum_position = prepare_empty_human_forum(&mut game, true);
    let food_before = game.food(Camp::Human);

    let events = game
        .apply(Action::RecruitVillager {
            building_position: forum_position,
        })
        .expect("villager should be recruitable on an empty allied forum");

    assert_eq!(
        events,
        vec![Event::UnitRecruited {
            unit_id: rusty_empires::UnitId(5),
            camp: Camp::Human,
            kind: UnitKind::Villager,
            position: forum_position,
        }]
    );
    assert_eq!(game.food(Camp::Human), food_before - 10);
    assert_eq!(
        game.unit_position(rusty_empires::UnitId(5)),
        Some(forum_position)
    );
    assert_eq!(
        game.unit_kind(rusty_empires::UnitId(5)),
        Some(UnitKind::Villager)
    );
}

#[test]
fn villager_cannot_be_recruited_without_food() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let forum_position = prepare_empty_human_forum(&mut game, false);

    assert_eq!(
        game.apply(Action::RecruitVillager {
            building_position: forum_position,
        }),
        Err(GameError::Recruit(RecruitError::NotEnoughFood))
    );
}

#[test]
fn villager_cannot_be_recruited_on_occupied_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let forum_position = prepare_occupied_human_forum(&mut game, true);

    assert_eq!(
        game.apply(Action::RecruitVillager {
            building_position: forum_position,
        }),
        Err(GameError::Recruit(RecruitError::Occupied))
    );
}

#[test]
fn villager_cannot_be_recruited_without_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);

    assert_eq!(
        game.apply(Action::RecruitVillager {
            building_position: GridPosition { x: 0, y: 0 },
        }),
        Err(GameError::Recruit(RecruitError::NoForum))
    );
}

#[test]
fn villager_cannot_be_recruited_from_enemy_forum() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let enemy_forum_position = prepare_enemy_forum(&mut game);

    assert_eq!(
        game.apply(Action::RecruitVillager {
            building_position: enemy_forum_position,
        }),
        Err(GameError::Recruit(RecruitError::NotOwnerTurn))
    );
}

fn prepare_empty_human_barracks(game: &mut Game, with_food: bool) -> GridPosition {
    let barracks_position = prepare_occupied_human_barracks(game, with_food);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    pass_turn_back_to_human(game);
    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 3 },
    })
    .expect("villager should be able to move away from the barracks onto the forum");

    barracks_position
}

fn prepare_empty_human_forum(game: &mut Game, with_food: bool) -> GridPosition {
    let forum_position = prepare_occupied_human_forum(game, with_food);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    pass_turn_back_to_human(game);
    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 4 },
    })
    .expect("villager should be able to move away from the forum");

    forum_position
}

fn prepare_occupied_human_forum(game: &mut Game, with_food: bool) -> GridPosition {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    if with_food {
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
    }

    let forum_position = GridPosition { x: 0, y: 3 };
    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: forum_position,
    })
    .expect("villager should be able to move to a non-resource tile");
    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("villager should be able to build a forum");
    assert_eq!(
        game.building_at(forum_position),
        Some((Camp::Human, BuildingKind::Forum))
    );

    forum_position
}

fn prepare_occupied_human_barracks(game: &mut Game, with_food: bool) -> GridPosition {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    if with_food {
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
    }

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
    assert_eq!(
        game.building_at(barracks_position),
        Some((Camp::Human, BuildingKind::Barracks))
    );

    barracks_position
}

fn pass_turn_back_to_human(game: &mut Game) {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");
}

fn prepare_enemy_barracks(game: &mut Game) -> GridPosition {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist");

    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: GridPosition { x: 8, y: 3 },
    })
    .expect("AI villager should be able to move to a non-resource tile");
    game.apply(Action::BuildForum {
        unit_id: ai_villager,
    })
    .expect("AI villager should be able to build a forum");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after building a forum");

    game.apply(Action::EndTurn)
        .expect("human should be able to end turn again");
    let enemy_barracks_position = GridPosition { x: 8, y: 2 };
    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: enemy_barracks_position,
    })
    .expect("AI villager should be able to move next to its forum");
    game.apply(Action::BuildBarracks {
        unit_id: ai_villager,
    })
    .expect("AI villager should be able to build a barracks next to its forum");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after building a barracks");

    enemy_barracks_position
}

fn prepare_enemy_forum(game: &mut Game) -> GridPosition {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist");
    let enemy_forum_position = GridPosition { x: 8, y: 3 };

    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: enemy_forum_position,
    })
    .expect("AI villager should be able to move to a non-resource tile");
    game.apply(Action::BuildForum {
        unit_id: ai_villager,
    })
    .expect("AI villager should be able to build a forum");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back after building a forum");

    enemy_forum_position
}
