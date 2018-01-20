use field::Field;

pub const PLAYER_RADIUS: f32 = 20.0;
pub const BULLET_RADIUS: f32 = 10.0;
pub const MAXIMUM_EXPLODE_RADIUS: f32 = 40.0;
pub const PLAYER_SPEED: f32 = 5.0;
pub const BULLET_SPEED: f32 = 25.0;

pub trait GameObject {
    fn pos(&self) -> Position;
    fn set_pos(&mut self, pos: Position);
    fn vector(&self) -> Vector;
    fn set_vector(&mut self, vector: Vector);
    fn move_next(&mut self) {
        let (vec_x, vec_y) = {
            let Vector { x, y } = self.vector();
            (x, y)
        };
        let Position { x, y } = self.pos();
        self.set_pos(Position {
            x: x + vec_x,
            y: y + vec_y,
        });
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Existing,
    Nil,
    Exploded,
}

pub struct Player {
    pub pos: Position,
    pub vector: Vector,
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

    fn vector(&self) -> Vector {
        self.vector
    }

    fn set_vector(&mut self, vector: Vector) {
        self.vector = vector
    }
}

#[derive(Clone)]
pub struct Enemy {
    pub pos: Position,
    pub vector: Vector,
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

    fn vector(&self) -> Vector {
        self.vector
    }

    fn set_vector(&mut self, vector: Vector) {
        self.vector = vector
    }
}

pub struct Bullet {
    pub pos: Position,
    pub vector: Vector,
    pub state: State,
}

impl GameObject for Bullet {
    fn pos(&self) -> Position {
        self.pos
    }

    fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    fn vector(&self) -> Vector {
        self.vector
    }

    fn set_vector(&mut self, vector: Vector) {
        self.vector = vector
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum ExtendDirection {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

pub enum Command {
    Move(ExtendDirection),
    Shot(Direction),
    Stay,
}
