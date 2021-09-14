use crate::{level::Level, player::Player};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    level: Level,
    player: Player,
}

impl State {
    /// Basically renders the current scene by returning the over
    pub fn render() -> [u64; 800] {
        let out = [0; 800];

        out
    }
}
