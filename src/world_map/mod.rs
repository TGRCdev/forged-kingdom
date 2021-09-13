mod plugin;
pub use plugin::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WorldMapState {
    Paused,
    Running
}