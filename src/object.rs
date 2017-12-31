pub const PLAYER_RADIUS: f32 = 20.0;
pub const BULLET_RADIUS: f32 = 10.0;
pub const MAXIMUM_EXPLODE_RADIUS: f32 = 25.0;

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

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Existing,
    Nil,
    Exploded,
}

pub struct Player {
    pub pos: Position,
    pub direction: Direction,
    pub state: State,
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

#[derive(Clone)]
pub struct Enemy {
    pub pos: Position,
    pub direction: Direction,
    pub state: State,
    pub explode_radius: f32,
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
    pub state: State,
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
