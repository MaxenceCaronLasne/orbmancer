#[derive(Debug)]
pub enum Error {
    Convertion,
    IndexTooBig,
    CellFull,
    NeighborNotFound,
    NoBalls,
    InventoryFull,
    TooManyNeighbors,
    NoLastState,
}
