use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Lingo {
    Berries,
    BerryGrowth,
    ChunkBorders,
    DisplayX,
    Kind,
    Latency,
    LoadedChunks,
    LoadedObjects,
    Logout,
    MousePosition,
    ObjectBounds,
    ObjectCollisions,
    PlaceVerb,
    Position,
    ServerTime,
    UiScale,
}
