use crate::{level::Level, player::Player};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    level: Level,
    player: Player,
}
