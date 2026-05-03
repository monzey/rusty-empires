use crate::core::resources::ResourceStockpile;
use crate::core::{BuildingKind, Camp, Event, Game, TradeError};

const TRADE_UNIT: i32 = 10;
const TRADE_RETURN: i32 = 5;

pub(crate) fn trade_gold_for_food(game: &mut Game, amount: i32) -> Result<Vec<Event>, TradeError> {
    validate_amount(amount)?;
    let camp = game.current_turn;

    if !has_market(game, camp) {
        return Err(TradeError::NoMarket);
    }

    if resources(game, camp).gold < amount {
        return Err(TradeError::NotEnoughGold);
    }

    let food_gained = traded_amount(amount);
    let (gold_total, food_total) = {
        let resources = resources_mut(game, camp);
        resources.gold -= amount;
        resources.food += food_gained;
        (resources.gold, resources.food)
    };

    Ok(vec![Event::GoldTradedForFood {
        camp,
        gold_spent: amount,
        food_gained,
        gold_total,
        food_total,
    }])
}

pub(crate) fn trade_food_for_gold(game: &mut Game, amount: i32) -> Result<Vec<Event>, TradeError> {
    validate_amount(amount)?;
    let camp = game.current_turn;

    if !has_market(game, camp) {
        return Err(TradeError::NoMarket);
    }

    if resources(game, camp).food < amount {
        return Err(TradeError::NotEnoughFood);
    }

    let gold_gained = traded_amount(amount);
    let (food_total, gold_total) = {
        let resources = resources_mut(game, camp);
        resources.food -= amount;
        resources.gold += gold_gained;
        (resources.food, resources.gold)
    };

    Ok(vec![Event::FoodTradedForGold {
        camp,
        food_spent: amount,
        gold_gained,
        food_total,
        gold_total,
    }])
}

fn validate_amount(amount: i32) -> Result<(), TradeError> {
    if amount <= 0 || amount % TRADE_UNIT != 0 {
        return Err(TradeError::InvalidAmount);
    }

    Ok(())
}

fn traded_amount(amount: i32) -> i32 {
    amount / TRADE_UNIT * TRADE_RETURN
}

fn has_market(game: &Game, camp: Camp) -> bool {
    game.buildings
        .iter()
        .any(|building| building.camp == camp && building.kind == BuildingKind::Market)
}

fn resources(game: &Game, camp: Camp) -> ResourceStockpile {
    match camp {
        Camp::Human => game.human_resources,
        Camp::Ai => game.ai_resources,
    }
}

fn resources_mut(game: &mut Game, camp: Camp) -> &mut ResourceStockpile {
    match camp {
        Camp::Human => &mut game.human_resources,
        Camp::Ai => &mut game.ai_resources,
    }
}
