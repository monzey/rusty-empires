use super::{GridPosition, UnitId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MoveUnit {
        unit_id: UnitId,
        to: GridPosition,
    },
    AttackUnit {
        attacker_id: UnitId,
        target_id: UnitId,
    },
    AttackBuilding {
        attacker_id: UnitId,
        target_position: GridPosition,
    },
    AttackWithBuilding {
        building_position: GridPosition,
        target_id: UnitId,
    },
    BuildGoldMine {
        unit_id: UnitId,
    },
    BuildFarm {
        unit_id: UnitId,
    },
    BuildForum {
        unit_id: UnitId,
    },
    BuildBarracks {
        unit_id: UnitId,
    },
    BuildMarket {
        unit_id: UnitId,
    },
    BuildUniversity {
        unit_id: UnitId,
    },
    BuildWatchtower {
        unit_id: UnitId,
    },
    RecruitSoldier {
        building_position: GridPosition,
    },
    RecruitArcher {
        building_position: GridPosition,
    },
    RecruitVillager {
        building_position: GridPosition,
    },
    TradeGoldForFood {
        amount: i32,
    },
    TradeFoodForGold {
        amount: i32,
    },
    ResearchAgriculture,
    ResearchMilitaryTraining,
    EndTurn,
    RunAiTurn,
}
