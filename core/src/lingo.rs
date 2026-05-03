use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Lingo {
    ActionPanelPosition,
    Age,
    Berries,
    BerryGrowth,
    Bottom,
    ChunkBorders,
    DisplayX,
    Growth,
    Kind,
    Latency,
    LoadedChunks,
    LoadedObjects,
    Logout,
    MousePosition,
    ObjectBounds,
    ObjectCollisions,
    PlaceVerb,
    Plants,
    Position,
    ServerTime,
    Top,
    UiScale,
}
