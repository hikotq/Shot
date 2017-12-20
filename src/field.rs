use glium::Display;
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use object::*;
use render::Render;

pub struct Field {
    display: Display,
    pub events_loop: EventsLoop,
    pub player: Player,
    pub bullet_list: Vec<Bullet>,
    pub enemy_list: Vec<Enemy>,
}

impl Field {
    pub fn new(width: u32, height: u32) -> Field {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new().with_dimensions(width, height);
        let context = ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        let player = Player {
            pos: Position { x: 70.0, y: 70.0 },
            direction: Direction {
                dir_x: 0.0,
                dir_y: 0.0,
            },
            remain_bullet: 0,
        };
        let mut bullet_list: Vec<Bullet> = Vec::new();
        let mut enemy_list: Vec<Enemy> = Vec::new();
        Field {
            display: display,
            events_loop: events_loop,
            player: player,
            bullet_list: bullet_list,
            enemy_list: enemy_list,
        }
    }

    pub fn update(&mut self) {
        self.player.move_next();
        for enemy in self.enemy_list.iter_mut() {
            enemy.move_next();
        }
        for bullet in self.bullet_list.iter_mut() {
            bullet.move_next();
        }
        self.detect_collision();
    }

    fn detect_collision(&mut self) {
        let player_pos = self.player.pos();
        for enemy in self.enemy_list.iter() {
            let enemy_pos = enemy.pos();
            if (player_pos.x - enemy_pos.x).powf(2.0) < PLAYER_RADIUS &&
                (player_pos.y - enemy_pos.y).powf(2.0) < PLAYER_RADIUS
            {
                self.player.pos = Position { x: 70.0, y: 70.0 };
            }
        }
    }

    pub fn draw(&self) {
        let mut render = Render::new(&self.display);
        render.clear_color(1.0, 1.0, 1.0, 1.0);
        let player: &Player = &self.player;
        let Position { x, y } = player.pos();
        render.draw_rectangle(Position { x: x, y: y }, 20);
        for enemy in self.enemy_list.iter() {
            let Position { x, y } = enemy.pos();
            render.draw_rectangle(Position { x: x, y: y }, 20);
        }
        render.finish();
    }
}
