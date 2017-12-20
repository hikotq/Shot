pub const PLAYER_RADIUS: f32 = 20.0;

pub trait GameObject {
    fn pos(&self) -> Position;
    fn set_pos(&mut self, pos: Position);
    fn direction(&self) -> Direction;
    fn set_direction(&mut self, direction: Direction);
    fn move_next(&mut self) {
        let Position { x, y } = self.pos();
        let Direction { dir_x, dir_y } = self.direction();
        self.set_pos(Position {
            x: x + dir_x,
            y: y + dir_y,
        });
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Direction {
    pub dir_x: f32,
    pub dir_y: f32,
}

pub struct Player {
    pub pos: Position,
    pub direction: Direction,
    pub remain_bullet: usize,
}

impl GameObject for Player {
    fn pos(&self) -> Position {
        self.pos
    }

    fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}

pub struct Enemy {
    pub pos: Position,
    pub direction: Direction,
}

impl GameObject for Enemy {
    fn pos(&self) -> Position {
        self.pos
    }

    fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}

pub struct Bullet {
    pub pos: Position,
    pub direction: Direction,
}

impl GameObject for Bullet {
    fn pos(&self) -> Position {
        self.pos
    }

    fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}
