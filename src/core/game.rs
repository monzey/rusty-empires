use super::buildings::BuildingState;
use super::map::NaturalResourceState;
use super::resources::ResourceStockpile;
use super::rules::{ai, combat, construction, economy, movement, recruitment, turns};
use super::units::{
    UnitState, SOLDIER_ATTACK, SOLDIER_ATTACK_RANGE, SOLDIER_DEFENSE, SOLDIER_HEALTH,
    SOLDIER_MOVE_RANGE, VILLAGER_ATTACK, VILLAGER_ATTACK_RANGE, VILLAGER_DEFENSE, VILLAGER_HEALTH,
    VILLAGER_MOVE_RANGE,
};
use super::{
    Action, BuildingKind, Camp, Event, GameError, GridPosition, NaturalResource, UnitId, UnitKind,
};

#[derive(Debug, Clone)]
pub struct Game {
    pub(crate) map_width: i32,
    pub(crate) map_height: i32,
    pub(crate) current_turn: Camp,
    pub(crate) units: Vec<UnitState>,
    pub(crate) natural_resources: Vec<NaturalResourceState>,
    pub(crate) buildings: Vec<BuildingState>,
    pub(crate) human_resources: ResourceStockpile,
    pub(crate) ai_resources: ResourceStockpile,
    pub(crate) next_unit_id: u32,
}

impl Game {
    pub fn new_single_player_vs_ai(map_width: i32, map_height: i32) -> Self {
        Self {
            map_width,
            map_height,
            current_turn: Camp::Human,
            units: vec![
                UnitState {
                    id: UnitId(1),
                    kind: UnitKind::Villager,
                    camp: Camp::Human,
                    position: GridPosition { x: 1, y: 3 },
                    has_moved: false,
                    has_acted: false,
                    health: VILLAGER_HEALTH,
                    attack: VILLAGER_ATTACK,
                    defense: VILLAGER_DEFENSE,
                    attack_range: VILLAGER_ATTACK_RANGE,
                    move_range: VILLAGER_MOVE_RANGE,
                },
                UnitState {
                    id: UnitId(2),
                    kind: UnitKind::Villager,
                    camp: Camp::Ai,
                    position: GridPosition {
                        x: map_width - 2,
                        y: map_height - 4,
                    },
                    has_moved: false,
                    has_acted: false,
                    health: VILLAGER_HEALTH,
                    attack: VILLAGER_ATTACK,
                    defense: VILLAGER_DEFENSE,
                    attack_range: VILLAGER_ATTACK_RANGE,
                    move_range: VILLAGER_MOVE_RANGE,
                },
                UnitState {
                    id: UnitId(3),
                    kind: UnitKind::Soldier,
                    camp: Camp::Human,
                    position: GridPosition { x: 4, y: 4 },
                    has_moved: false,
                    has_acted: false,
                    health: SOLDIER_HEALTH,
                    attack: SOLDIER_ATTACK,
                    defense: SOLDIER_DEFENSE,
                    attack_range: SOLDIER_ATTACK_RANGE,
                    move_range: SOLDIER_MOVE_RANGE,
                },
                UnitState {
                    id: UnitId(4),
                    kind: UnitKind::Soldier,
                    camp: Camp::Ai,
                    position: GridPosition { x: 5, y: 4 },
                    has_moved: false,
                    has_acted: false,
                    health: SOLDIER_HEALTH,
                    attack: SOLDIER_ATTACK,
                    defense: SOLDIER_DEFENSE,
                    attack_range: SOLDIER_ATTACK_RANGE,
                    move_range: SOLDIER_MOVE_RANGE,
                },
            ],
            natural_resources: vec![
                NaturalResourceState {
                    kind: NaturalResource::GoldDeposit,
                    position: GridPosition { x: 1, y: 3 },
                },
                NaturalResourceState {
                    kind: NaturalResource::Field,
                    position: GridPosition { x: 2, y: 3 },
                },
                NaturalResourceState {
                    kind: NaturalResource::GoldDeposit,
                    position: GridPosition {
                        x: map_width - 2,
                        y: map_height - 4,
                    },
                },
                NaturalResourceState {
                    kind: NaturalResource::Field,
                    position: GridPosition {
                        x: map_width - 3,
                        y: map_height - 4,
                    },
                },
            ],
            buildings: Vec::new(),
            human_resources: ResourceStockpile::default(),
            ai_resources: ResourceStockpile::default(),
            next_unit_id: 5,
        }
    }

    pub fn apply(&mut self, action: Action) -> Result<Vec<Event>, GameError> {
        if let Some(winner) = self.winner() {
            return Err(GameError::GameOver { winner });
        }

        match action {
            Action::MoveUnit { unit_id, to } => {
                movement::move_unit(self, unit_id, to).map_err(GameError::Move)
            }
            Action::AttackUnit {
                attacker_id,
                target_id,
            } => combat::attack_unit(self, attacker_id, target_id).map_err(GameError::Combat),
            Action::BuildGoldMine { unit_id } => {
                construction::build_gold_mine(self, unit_id).map_err(GameError::Build)
            }
            Action::BuildFarm { unit_id } => {
                construction::build_farm(self, unit_id).map_err(GameError::Build)
            }
            Action::BuildForum { unit_id } => {
                construction::build_forum(self, unit_id).map_err(GameError::Build)
            }
            Action::BuildBarracks { unit_id } => {
                construction::build_barracks(self, unit_id).map_err(GameError::Build)
            }
            Action::RecruitSoldier { building_position } => {
                recruitment::recruit_soldier(self, building_position).map_err(GameError::Recruit)
            }
            Action::RecruitVillager { building_position } => {
                recruitment::recruit_villager(self, building_position).map_err(GameError::Recruit)
            }
            Action::EndTurn => turns::end_human_turn(self).map_err(GameError::Turn),
            Action::RunAiTurn => ai::run_ai_turn(self).map_err(GameError::Turn),
        }
    }

    pub fn current_turn(&self) -> Camp {
        self.current_turn
    }

    pub fn villager_id(&self, camp: Camp) -> Option<UnitId> {
        self.units
            .iter()
            .find(|unit| unit.camp == camp && unit.kind == UnitKind::Villager)
            .map(|unit| unit.id)
    }

    pub fn soldier_id(&self, camp: Camp) -> Option<UnitId> {
        self.units
            .iter()
            .find(|unit| unit.camp == camp && unit.kind == UnitKind::Soldier)
            .map(|unit| unit.id)
    }

    pub fn villager_position(&self, camp: Camp) -> Option<GridPosition> {
        self.units
            .iter()
            .find(|unit| unit.camp == camp && unit.kind == UnitKind::Villager)
            .map(|unit| unit.position)
    }

    pub fn soldier_position(&self, camp: Camp) -> Option<GridPosition> {
        self.units
            .iter()
            .find(|unit| unit.camp == camp && unit.kind == UnitKind::Soldier)
            .map(|unit| unit.position)
    }

    pub fn unit_position(&self, unit_id: UnitId) -> Option<GridPosition> {
        self.units
            .iter()
            .find(|unit| unit.id == unit_id)
            .map(|unit| unit.position)
    }

    pub fn unit_has_acted(&self, unit_id: UnitId) -> Option<bool> {
        self.units
            .iter()
            .find(|unit| unit.id == unit_id)
            .map(|unit| unit.has_acted)
    }

    pub fn unit_health(&self, unit_id: UnitId) -> Option<i32> {
        self.units
            .iter()
            .find(|unit| unit.id == unit_id)
            .map(|unit| unit.health)
    }

    pub fn unit_kind(&self, unit_id: UnitId) -> Option<UnitKind> {
        self.units
            .iter()
            .find(|unit| unit.id == unit_id)
            .map(|unit| unit.kind)
    }

    pub fn natural_resource_at(&self, position: GridPosition) -> Option<NaturalResource> {
        self.natural_resources
            .iter()
            .find(|resource| resource.position == position)
            .map(|resource| resource.kind)
    }

    pub fn building_at(&self, position: GridPosition) -> Option<(Camp, BuildingKind)> {
        self.buildings
            .iter()
            .find(|building| building.position == position)
            .map(|building| (building.camp, building.kind))
    }

    pub fn gold(&self, camp: Camp) -> i32 {
        economy::resources(self, camp).gold
    }

    pub fn food(&self, camp: Camp) -> i32 {
        economy::resources(self, camp).food
    }

    pub fn winner(&self) -> Option<Camp> {
        let human_alive = self.has_assets(Camp::Human);
        let ai_alive = self.has_assets(Camp::Ai);

        match (human_alive, ai_alive) {
            (true, false) => Some(Camp::Human),
            (false, true) => Some(Camp::Ai),
            _ => None,
        }
    }

    pub(crate) fn is_inside_map(&self, position: GridPosition) -> bool {
        position.x >= 0
            && position.x < self.map_width
            && position.y >= 0
            && position.y < self.map_height
    }

    fn has_assets(&self, camp: Camp) -> bool {
        self.units.iter().any(|unit| unit.camp == camp)
            || self.buildings.iter().any(|building| building.camp == camp)
    }
}
