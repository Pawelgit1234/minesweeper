use crate::settings;
use crate::objects::cursor::Cursor;
use crate::objects::cell::Cell;

pub struct Game {
    cursor: Cursor,
    cells: [[Cell; settings::FIELD_WIDTH]; settings::FIELD_HEIGHT],
}

impl Game {
    
}