use crate::settings;

pub struct Cursor {
    x: u8,
    y: u8,
}

// smallest coordinates are (1|1)

impl Cursor {
    pub fn move_down(&mut self) {
        if self.y != settings::FIELD_WIDTH {
            self.y += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y != 1  {
            self.y -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x != settings::FIELD_HEIGHT  {
            self.x += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x != 1  {
            self.x -= 1;
        }
    }

    pub fn x(&mut self) {
        self.x
    }

    pub fn y(&mut self) {
        self.y
    }
}