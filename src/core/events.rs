use super::{BuildingKind, Camp, GridPosition, UnitId, UnitKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    UnitMoved {
        unit_id: UnitId,
        from: GridPosition,
        to: GridPosition,
    },
    UnitActed {
        unit_id: UnitId,
    },
    BuildingActed {
        camp: Camp,
        kind: BuildingKind,
        position: GridPosition,
    },
    UnitDamaged {
        unit_id: UnitId,
        amount: i32,
        remaining_health: i32,
    },
    UnitDefeated {
        unit_id: UnitId,
    },
    BuildingDamaged {
        camp: Camp,
        kind: BuildingKind,
        position: GridPosition,
        amount: i32,
        remaining_health: i32,
    },
    BuildingDestroyed {
        camp: Camp,
        kind: BuildingKind,
        position: GridPosition,
    },
    UnitRecruited {
        unit_id: UnitId,
        camp: Camp,
        kind: UnitKind,
        position: GridPosition,
    },
    BuildingConstructed {
        camp: Camp,
        kind: BuildingKind,
        position: GridPosition,
    },
    GoldProduced {
        camp: Camp,
        amount: i32,
        total: i32,
    },
    FoodProduced {
        camp: Camp,
        amount: i32,
        total: i32,
    },
    TechnologyProduced {
        camp: Camp,
        amount: i32,
        total: i32,
    },
    GoldTradedForFood {
        camp: Camp,
        gold_spent: i32,
        food_gained: i32,
        gold_total: i32,
        food_total: i32,
    },
    FoodTradedForGold {
        camp: Camp,
        food_spent: i32,
        gold_gained: i32,
        food_total: i32,
        gold_total: i32,
    },
    TechnologyResearched {
        camp: Camp,
        name: &'static str,
    },
    TurnChanged {
        from: Camp,
        to: Camp,
    },
    GameWon {
        camp: Camp,
    },
}
