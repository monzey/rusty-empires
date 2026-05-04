use rusty_empires::{Camp, Faction, Game};

#[test]
fn new_game_uses_default_factions() {
    let game = Game::new_single_player_vs_ai(10, 8);

    assert_eq!(game.faction(Camp::Human), Faction::Valdorian);
    assert_eq!(game.faction(Camp::Ai), Faction::Kharzun);
}

#[test]
fn new_game_can_choose_human_and_ai_factions() {
    let game = Game::new_single_player(Faction::Sylvans, Faction::Necrarchs, 10, 8);

    assert_eq!(game.faction(Camp::Human), Faction::Sylvans);
    assert_eq!(game.faction(Camp::Ai), Faction::Necrarchs);
}
