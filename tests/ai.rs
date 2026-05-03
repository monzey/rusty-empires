use rusty_empires::{Action, BuildingKind, Camp, Event, Game, GridPosition, UnitKind};

#[test]
fn ai_recruits_soldier_from_empty_barracks_when_it_has_food() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    let barracks_position = prepare_empty_ai_barracks_with_food(&mut game);

    let events = game
        .apply(Action::RunAiTurn)
        .expect("AI should run its turn and recruit from an empty barracks");

    assert!(events.contains(&Event::UnitRecruited {
        unit_id: rusty_empires::UnitId(5),
        camp: Camp::Ai,
        kind: UnitKind::Soldier,
        position: barracks_position,
    }));
    assert_eq!(
        game.unit_kind(rusty_empires::UnitId(5)),
        Some(UnitKind::Soldier)
    );
    assert_eq!(
        game.unit_position(rusty_empires::UnitId(5)),
        Some(barracks_position)
    );
}

#[test]
fn ai_does_not_recruit_soldier_from_occupied_barracks() {
    let mut game = Game::new_single_player_vs_ai(10, 8);
    prepare_occupied_ai_barracks_with_food(&mut game);

    let events = game
        .apply(Action::RunAiTurn)
        .expect("AI should run its turn without recruiting from an occupied barracks");

    assert!(events
        .iter()
        .all(|event| !matches!(event, Event::UnitRecruited { .. })));
}

fn prepare_empty_ai_barracks_with_food(game: &mut Game) -> GridPosition {
    let (barracks_position, forum_position, ai_villager) =
        prepare_occupied_ai_barracks_with_food(game);

    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: forum_position,
    })
    .expect("AI villager should be able to move away from the barracks");

    barracks_position
}

fn prepare_occupied_ai_barracks_with_food(
    game: &mut Game,
) -> (GridPosition, GridPosition, rusty_empires::UnitId) {
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
    let ai_villager = game
        .villager_id(Camp::Ai)
        .expect("AI villager should exist");

    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: GridPosition { x: 7, y: 4 },
    })
    .expect("AI villager should be able to move onto the field");
    game.apply(Action::BuildFarm {
        unit_id: ai_villager,
    })
    .expect("AI villager should be able to build a farm");
    pass_turn_back_to_ai(game);

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
    pass_turn_back_to_ai(game);

    let barracks_position = GridPosition { x: 8, y: 2 };
    game.apply(Action::MoveUnit {
        unit_id: ai_villager,
        to: barracks_position,
    })
    .expect("AI villager should be able to move next to its forum");
    game.apply(Action::BuildBarracks {
        unit_id: ai_villager,
    })
    .expect("AI villager should be able to build a barracks next to its forum");
    assert_eq!(
        game.building_at(barracks_position),
        Some((Camp::Ai, BuildingKind::Barracks))
    );
    pass_turn_back_to_ai(game);

    (barracks_position, forum_position, ai_villager)
}

fn pass_turn_back_to_ai(game: &mut Game) {
    game.apply(Action::RunAiTurn)
        .expect("AI should pass the turn back");
    game.apply(Action::EndTurn)
        .expect("human should be able to end turn");
}
