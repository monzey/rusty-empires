use super::{BuildingKind, Camp, GridPosition, UnitId};

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
    UnitDamaged {
        unit_id: UnitId,
        amount: i32,
        remaining_health: i32,
    },
    UnitDefeated {
        unit_id: UnitId,
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
    TurnChanged {
        from: Camp,
        to: Camp,
    },
}
