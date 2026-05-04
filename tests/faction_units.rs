use rusty_empires::{
    Action, BuildingKind, Camp, Event, Faction, Game, GridPosition, UnitId, UnitKind,
};

#[test]
fn valdorian_recruits_legionary_as_unique_unit() {
    unique_unit_can_be_recruited_for_faction(
        Faction::Valdorian,
        UnitKind::ValdorianLegionary,
        75,
        55,
    );
}

#[test]
fn kharzun_recruits_shieldbreaker_as_unique_unit() {
    unique_unit_can_be_recruited_for_faction(
        Faction::Kharzun,
        UnitKind::KharzunShieldbreaker,
        70,
        65,
    );
}

#[test]
fn sylvans_recruit_pathfinder_as_unique_unit() {
    unique_unit_can_be_recruited_for_faction(Faction::Sylvans, UnitKind::ElyrPathfinder, 55, 45);
}

#[test]
fn necrarchs_recruit_bone_servant_as_unique_unit() {
    unique_unit_can_be_recruited_for_faction(
        Faction::Necrarchs,
        UnitKind::ObsidianBoneServant,
        0,
        25,
    );
}

fn unique_unit_can_be_recruited_for_faction(
    faction: Faction,
    expected_kind: UnitKind,
    expected_food_cost: i32,
    expected_gold_cost: i32,
) {
    let mut game = Game::new_single_player(faction, Faction::Kharzun, 10, 8);
    let barracks_position = prepare_empty_human_barracks(&mut game);
    let food_before = game.food(Camp::Human);
    let gold_before = game.gold(Camp::Human);

    let events = game
        .apply(Action::RecruitUniqueUnit {
            building_position: barracks_position,
        })
        .expect("unique unit should be recruitable from an empty allied barracks");

    assert_eq!(
        events,
        vec![Event::UnitRecruited {
            unit_id: UnitId(5),
            camp: Camp::Human,
            kind: expected_kind,
            position: barracks_position,
        }]
    );
    assert_eq!(game.unit_kind(UnitId(5)), Some(expected_kind));
    assert_eq!(game.food(Camp::Human), food_before - expected_food_cost);
    assert_eq!(game.gold(Camp::Human), gold_before - expected_gold_cost);
}

fn prepare_empty_human_barracks(game: &mut Game) -> GridPosition {
    let human_villager = game
        .villager_id(Camp::Human)
        .expect("human villager should exist");
    let forum_position = GridPosition { x: 0, y: 3 };
    let barracks_position = GridPosition { x: 0, y: 4 };

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: forum_position,
    })
    .expect("villager should move to forum site");
    game.apply(Action::BuildForum {
        unit_id: human_villager,
    })
    .expect("villager should build forum");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: barracks_position,
    })
    .expect("villager should move to barracks site");
    game.apply(Action::BuildBarracks {
        unit_id: human_villager,
    })
    .expect("villager should build barracks");
    pass_turn_back_to_human(game);

    game.apply(Action::MoveUnit {
        unit_id: human_villager,
        to: forum_position,
    })
    .expect("villager should move away from barracks");
    assert_eq!(
        game.building_at(barracks_position),
        Some((Camp::Human, BuildingKind::Barracks))
    );

    barracks_position
}

fn pass_turn_back_to_human(game: &mut Game) {
    game.apply(Action::EndTurn).expect("human should end turn");
    game.apply(Action::RunAiTurn)
        .expect("AI should pass turn back");
}
