use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;

pub struct EnemyStatus {
    pub alive: u32,
    pub killed: u32,
}

#[derive(Resource)]
pub struct WaveStats {
    pub wave: u32,
    pub enemies: HashMap<String, EnemyStatus>,
}

impl Default for WaveStats {
    fn default() -> Self {
        Self {
            wave: 1,
            enemies: HashMap::default(),
        }
    }
}