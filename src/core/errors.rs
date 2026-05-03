#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    GameOver { winner: crate::core::Camp },
    Move(MoveError),
    Combat(CombatError),
    Build(BuildError),
    Recruit(RecruitError),
    Trade(TradeError),
    Research(ResearchError),
    Turn(TurnError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResearchError {
    NoUniversity,
    NotEnoughTechnologyPoints,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeError {
    NoMarket,
    InvalidAmount,
    NotEnoughGold,
    NotEnoughFood,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecruitError {
    NoBarracks,
    NoForum,
    NotOwnerTurn,
    Occupied,
    NotEnoughFood,
    NotEnoughGold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatError {
    NoAttacker,
    NoTarget,
    NotAttackerTurn,
    AlreadyActed,
    FriendlyTarget,
    TargetNotVisible,
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
    EnemyBuilding,
    OutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnError {
    NotHumanTurn,
    NotAiTurn,
}
