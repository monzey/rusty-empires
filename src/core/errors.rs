#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    Move(MoveError),
    Combat(CombatError),
    Build(BuildError),
    Turn(TurnError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatError {
    NoAttacker,
    NoTarget,
    NotAttackerTurn,
    AlreadyActed,
    FriendlyTarget,
    OutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    NoUnit,
    NotBuilder,
    NotUnitTurn,
    AlreadyActed,
    NoGoldDeposit,
    NoField,
    NaturalResourcePresent,
    OccupiedByBuilding,
    NoAdjacentForum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    NoUnit,
    NotUnitTurn,
    AlreadyActed,
    OutsideMap,
    Occupied,
    OutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnError {
    NotHumanTurn,
    NotAiTurn,
}
