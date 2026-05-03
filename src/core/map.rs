use super::GridPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaturalResource {
    GoldDeposit,
    Field,
}

#[derive(Debug, Clone)]
pub(crate) struct NaturalResourceState {
    pub(crate) kind: NaturalResource,
    pub(crate) position: GridPosition,
}
