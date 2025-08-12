#[derive(Debug)]
pub enum Error {
    Convertion,
    IndexTooBig,
    CellFull,
    NoBalls,
    TooManyNeighbors,
    NoLastState,
}
