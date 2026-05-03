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
    RecruitSoldier {
        building_position: GridPosition,
    },
    RecruitVillager {
        building_position: GridPosition,
    },
    EndTurn,
    RunAiTurn,
}
