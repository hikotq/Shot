use std::hash::{Hash, Hasher};

use field::Field;

pub const PLAYER_RADIUS: f32 = 20.0;
pub const BULLET_RADIUS: f32 = 10.0;
pub const MAXIMUM_EXPLODE_RADIUS: f32 = 40.0;
pub const PLAYER_SPEED: f32 = 5.0;
pub const BULLET_SPEED: f32 = 10.0;
pub const MAXIMUM_BULLET: usize = 5;
pub const KILLING_POINT: u64 = 100;


pub trait Mover {
    fn pos(&self) -> Position;
    fn set_pos(&mut self, pos: Position);
    fn vector(&self) -> Vector;
    fn set_vector(&mut self, vector: Vector);
}

pub trait Move: Mover {
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
    fn update(&mut self) {
        self.move_next();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = self.x as i32;
        let y = self.y as i32;
        x.hash(state);
        y.hash(state);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = (self.x * 100.0) as i32;
        let y = (self.y * 100.0) as i32;
        x.hash(state);
        y.hash(state);
    }
}

#[derive(Copy, Clone, PartialEq, Hash)]
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
    pub bullet_timer: usize,
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.remain_bullet.hash(state);
    }
}

impl Mover for Player {
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

impl Move for Player {
    fn update(&mut self) {
        self.move_next();
        if self.bullet_timer == 0 {
            if self.remain_bullet != MAXIMUM_BULLET {
                self.remain_bullet += 1;
            }
            self.bullet_timer = 30;
        } else {
            self.bullet_timer -= 1;
        }
    }
}

#[derive(Clone)]
pub struct Enemy {
    pub pos: Position,
    pub vector: Vector,
    pub state: State,
}

impl Hash for Enemy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl Mover for Enemy {
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

impl Move for Enemy {}

pub struct Explosion {
    pub pos: Position,
    pub radius: f32,
    pub chain: u64,
}

#[derive(Hash)]
pub struct Bullet {
    pub pos: Position,
    pub vector: Vector,
    pub state: State,
}

impl Mover for Bullet {
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

impl Move for Bullet {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Command {
    Move(ExtendDirection),
    Shot(Direction),
    Stay,
}

use std::slice::Iter;
impl Command {
    pub fn iterator() -> Iter<'static, Command> {
        use self::Command::*;
        use self::ExtendDirection;
        use self::Direction;
        static COMMANDS: [Command; 13] = [
            Move(ExtendDirection::Left),
            Move(ExtendDirection::Right),
            Move(ExtendDirection::Up),
            Move(ExtendDirection::Down),
            Move(ExtendDirection::LeftUp),
            Move(ExtendDirection::RightUp),
            Move(ExtendDirection::LeftDown),
            Move(ExtendDirection::RightDown),
            Shot(Direction::Left),
            Shot(Direction::Right),
            Shot(Direction::Up),
            Shot(Direction::Down),
            Stay,
        ];
        COMMANDS.into_iter()
    }
}
