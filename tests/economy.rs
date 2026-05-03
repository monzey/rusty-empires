use rusty_empires::{Action, Camp, Event, Game, GridPosition};

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

    let events = game
        .apply(Action::EndTurn)
        .expect("ending the owner turn should produce gold");

    assert_eq!(game.gold(Camp::Human), 10);
    assert_eq!(game.gold(Camp::Ai), 0);
    assert!(events.contains(&Event::GoldProduced {
        camp: Camp::Human,
        amount: 10,
        total: 10,
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

    let events = game
        .apply(Action::EndTurn)
        .expect("ending the owner turn should produce food");

    assert_eq!(game.food(Camp::Human), 10);
    assert_eq!(game.food(Camp::Ai), 0);
    assert!(events.contains(&Event::FoodProduced {
        camp: Camp::Human,
        amount: 10,
        total: 10,
    }));
}
