use super::{Camp, GridPosition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingKind {
    GoldMine,
    Farm,
    Forum,
    Barracks,
}

#[derive(Debug, Clone)]
pub(crate) struct BuildingState {
    pub(crate) camp: Camp,
    pub(crate) kind: BuildingKind,
    pub(crate) position: GridPosition,
}
