pub enum CellType {
    EMPTY,
    MINE,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
}

pub enum CellState {
    OPENED,
    CLOSED,
    FLAG,
}

pub struct Cell {
    cell_type: CellType,
    cell_state: CellState,
}