use super::{Camp, GridPosition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingKind {
    GoldMine,
    Farm,
    Forum,
    Barracks,
    Market,
    University,
}

pub(crate) const GOLD_MINE_HEALTH: i32 = 10;
pub(crate) const FARM_HEALTH: i32 = 10;
pub(crate) const FORUM_HEALTH: i32 = 20;
pub(crate) const BARRACKS_HEALTH: i32 = 15;
pub(crate) const MARKET_HEALTH: i32 = 15;
pub(crate) const UNIVERSITY_HEALTH: i32 = 15;

pub(crate) fn health_for_kind(kind: BuildingKind) -> i32 {
    match kind {
        BuildingKind::GoldMine => GOLD_MINE_HEALTH,
        BuildingKind::Farm => FARM_HEALTH,
        BuildingKind::Forum => FORUM_HEALTH,
        BuildingKind::Barracks => BARRACKS_HEALTH,
        BuildingKind::Market => MARKET_HEALTH,
        BuildingKind::University => UNIVERSITY_HEALTH,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BuildingState {
    pub(crate) camp: Camp,
    pub(crate) kind: BuildingKind,
    pub(crate) position: GridPosition,
    pub(crate) health: i32,
}
