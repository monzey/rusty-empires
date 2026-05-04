use rusty_empires::{Action, Camp, Event, Game, GameError, GridPosition, ResearchError};

#[test]
fn military_training_consumes_technology_points_and_improves_soldier_attacks() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_human_university_with_technology_points(&mut game);
    let human_soldier = game
        .soldier_id(Camp::Human)
        .expect("human soldier should exist");
    let ai_soldier = game.soldier_id(Camp::Ai).expect("AI soldier should exist");

    let research_events = game
        .apply(Action::ResearchMilitaryTraining)
        .expect("human should be able to research military training");

    assert_eq!(game.technology_points(Camp::Human), 0);
    assert!(research_events.contains(&Event::TechnologyResearched {
        camp: Camp::Human,
        name: "MilitaryTraining",
    }));

    let attack_events = game
        .apply(Action::AttackUnit {
            attacker_id: human_soldier,
            target_id: ai_soldier,
        })
        .expect("trained human soldier should still be able to attack");

    assert!(attack_events.contains(&Event::UnitDamaged {
        unit_id: ai_soldier,
        amount: 5,
        remaining_health: 5,
    }));
}

#[test]
fn military_training_requires_university() {
    let mut game = Game::new_single_player_vs_ai(10, 8);

    assert_eq!(
        game.apply(Action::ResearchMilitaryTraining),
        Err(GameError::Research(ResearchError::NoUniversity))
    );
}

#[test]
fn military_training_requires_enough_technology_points() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_human_university_without_technology_points(&mut game);

    assert_eq!(
        game.apply(Action::ResearchMilitaryTraining),
        Err(GameError::Research(
            ResearchError::NotEnoughTechnologyPoints
        ))
    );
}

#[test]
fn agriculture_consumes_technology_and_food_then_improves_farm_yields() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_human_farm_and_university_with_technology_points(&mut game, 8);
    let food_before = game.food(Camp::Human);

    let research_events = game
        .apply(Action::ResearchAgriculture)
        .expect("human should be able to research agriculture with a farm and university");

    assert_eq!(game.technology_points(Camp::Human), 0);
    assert_eq!(game.food(Camp::Human), food_before - 60);
    assert!(research_events.contains(&Event::TechnologyResearched {
        camp: Camp::Human,
        name: "Agriculture",
    }));

    let food_after_research = game.food(Camp::Human);
    let events = game
        .apply(Action::EndTurn)
        .expect("ending turn should produce improved farm food");

    assert_eq!(game.food(Camp::Human), food_after_research + 25);
    assert!(events.contains(&Event::FoodProduced {
        camp: Camp::Human,
        amount: 25,
        total: food_after_research + 25,
    }));
}

#[test]
fn agriculture_cannot_be_researched_twice() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_human_farm_and_university_with_technology_points(&mut game, 8);

    game.apply(Action::ResearchAgriculture)
        .expect("first agriculture research should be valid");
    for _ in 0..8 {
        pass_turn_back_to_human(&mut game);
    }
    let food_before = game.food(Camp::Human);
    let technology_before = game.technology_points(Camp::Human);

    assert_eq!(
        game.apply(Action::ResearchAgriculture),
        Err(GameError::Research(ResearchError::AlreadyResearched))
    );
    assert_eq!(game.food(Camp::Human), food_before);
    assert_eq!(game.technology_points(Camp::Human), technology_before);
}

fn prepare_human_university_with_technology_points(game: &mut Game) {
    prepare_human_university_without_technology_points(game);
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn and produce technology points");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");
}

fn prepare_human_farm_and_university_with_technology_points(game: &mut Game, turns: usize) {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 2, y: 3 },
    })
    .expect("human villager should be able to move onto the field");
    game.apply(Action::BuildFarm {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a farm");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 3 },
    })
    .expect("human villager should be able to move to a forum site");
    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a forum");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 4 },
    })
    .expect("human villager should be able to move adjacent to the forum");
    pass_turn_back_to_human(game);
    game.apply(Action::BuildUniversity {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a university");

    for _ in 0..turns {
        pass_turn_back_to_human(game);
    }
}

fn prepare_human_university_without_technology_points(game: &mut Game) {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 3 },
    })
    .expect("human villager should be able to move to a non-resource tile");
    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a forum");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: GridPosition { x: 0, y: 4 },
    })
    .expect("human villager should be able to move adjacent to the forum");
    pass_turn_back_to_human(game);
    game.apply(Action::BuildUniversity {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a university");
}

fn pass_turn_back_to_human(game: &mut Game) {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");
}
