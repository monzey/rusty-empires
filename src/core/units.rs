use super::{Camp, GridPosition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u32);

pub(crate) const VILLAGER_HEALTH: i32 = 6;
pub(crate) const VILLAGER_ATTACK: i32 = 4;
pub(crate) const VILLAGER_DEFENSE: i32 = 1;
pub(crate) const VILLAGER_ATTACK_RANGE: i32 = 1;
pub(crate) const VILLAGER_MOVE_RANGE: i32 = 5;

#[derive(Debug, Clone)]
pub(crate) struct UnitState {
    pub(crate) id: UnitId,
    pub(crate) camp: Camp,
    pub(crate) position: GridPosition,
    pub(crate) has_acted: bool,
    pub(crate) health: i32,
    pub(crate) attack: i32,
    pub(crate) defense: i32,
    pub(crate) attack_range: i32,
    pub(crate) move_range: i32,
}
