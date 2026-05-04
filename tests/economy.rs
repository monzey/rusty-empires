use rusty_empires::{Action, Camp, Event, Game, GameError, GridPosition, TradeError};

#[test]
fn gold_mine_produces_gold_at_owner_end_of_turn() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist at game start");

    game.apply(Action::BuildGoldMine {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a gold mine on the initial deposit");
    let human_gold_before = game.gold(Camp::Human);
    let ai_gold_before = game.gold(Camp::Ai);

    let events = game
        .apply(Action::EndTurn)
        .expect("ending the owner turn should produce gold");

    assert_eq!(game.gold(Camp::Human), human_gold_before + 10);
    assert_eq!(game.gold(Camp::Ai), ai_gold_before);
    assert!(events.contains(&Event::GoldProduced {
        camp: Camp::Human,
        amount: 10,
        total: human_gold_before + 10,
    }));
}

#[test]
fn farm_produces_food_at_owner_end_of_turn() {
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
    game.apply(Action::BuildFarm {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a farm on a field");
    let human_food_before = game.food(Camp::Human);
    let ai_food_before = game.food(Camp::Ai);

    let events = game
        .apply(Action::EndTurn)
        .expect("ending the owner turn should produce food");

    assert_eq!(game.food(Camp::Human), human_food_before + 10);
    assert_eq!(game.food(Camp::Ai), ai_food_before);
    assert!(events.contains(&Event::FoodProduced {
        camp: Camp::Human,
        amount: 10,
        total: human_food_before + 10,
    }));
}

#[test]
fn university_produces_technology_points_at_owner_end_of_turn() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_human_university(&mut game);

    let events = game
        .apply(Action::EndTurn)
        .expect("ending the owner turn should produce technology points");

    assert_eq!(game.technology_points(Camp::Human), 10);
    assert_eq!(game.technology_points(Camp::Ai), 0);
    assert!(events.contains(&Event::TechnologyProduced {
        camp: Camp::Human,
        amount: 10,
        total: 10,
    }));
}

#[test]
fn human_can_trade_gold_for_food_with_allied_market() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_human_market_with_gold(&mut game);
    let gold_before = game.gold(Camp::Human);
    let food_before = game.food(Camp::Human);

    let events = game
        .apply(Action::TradeGoldForFood { amount: 10 })
        .expect("human should be able to trade gold for food with an allied market");

    assert_eq!(game.gold(Camp::Human), gold_before - 10);
    assert_eq!(game.food(Camp::Human), food_before + 5);
    assert!(events.contains(&Event::GoldTradedForFood {
        camp: Camp::Human,
        gold_spent: 10,
        food_gained: 5,
        gold_total: gold_before - 10,
        food_total: food_before + 5,
    }));
}

#[test]
fn human_can_trade_food_for_gold_with_allied_market() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_human_market_with_gold(&mut game);
    game.apply(Action::TradeGoldForFood { amount: 20 })
        .expect("human should be able to create enough food for the reverse trade");
    let gold_before = game.gold(Camp::Human);
    let food_before = game.food(Camp::Human);

    let events = game
        .apply(Action::TradeFoodForGold { amount: 10 })
        .expect("human should be able to trade food for gold with an allied market");

    assert_eq!(game.food(Camp::Human), food_before - 10);
    assert_eq!(game.gold(Camp::Human), gold_before + 5);
    assert!(events.contains(&Event::FoodTradedForGold {
        camp: Camp::Human,
        food_spent: 10,
        gold_gained: 5,
        food_total: food_before - 10,
        gold_total: gold_before + 5,
    }));
}

#[test]
fn trade_requires_an_allied_market() {
    let mut game = Game::new_single_player_vs_ai(10, 8);

    assert_eq!(
        game.apply(Action::TradeGoldForFood { amount: 10 }),
        Err(GameError::Trade(TradeError::NoMarket))
    );
}

#[test]
fn trade_requires_enough_resources() {
    let mut game = Game::new_single_player_vs_ai_with_resources(10, 8, 410, 180, 1000, 500);
    prepare_human_market_without_resources(&mut game);

    assert_eq!(
        game.apply(Action::TradeGoldForFood { amount: 10 }),
        Err(GameError::Trade(TradeError::NotEnoughGold))
    );
    assert_eq!(
        game.apply(Action::TradeFoodForGold { amount: 10 }),
        Err(GameError::Trade(TradeError::NotEnoughFood))
    );
}

fn prepare_human_market_with_gold(game: &mut Game) {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");

    game.apply(Action::BuildGoldMine {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a gold mine");
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn and produce gold");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");

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
    game.apply(Action::BuildMarket {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a market");
}

fn prepare_human_university(game: &mut Game) {
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

fn prepare_human_market_without_resources(game: &mut Game) {
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
    game.apply(Action::BuildMarket {
        unit_id: human_villager,
    })
    .expect("human villager should be able to build a market");
}

fn pass_turn_back_to_human(game: &mut Game) {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");
}
