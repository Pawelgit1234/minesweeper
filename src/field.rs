mod settings;
mod cursor;

use cursor::Cursor;

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

pub struct Field {
    cells: [[Cell; settings::FIELD_WIDTH]; settings::FIELD_HEIGHT],
    cursor: Cursor,
}