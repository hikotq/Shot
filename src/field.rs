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
        let player_pos = self.player.pos;
        //プレイヤーと敵の当たり判定
        for enemy in self.enemy_list.iter() {
            let enemy_pos = enemy.pos;
            if (player_pos.x - enemy_pos.x).powf(2.0) < PLAYER_RADIUS &&
                (player_pos.y - enemy_pos.y).powf(2.0) < PLAYER_RADIUS
            {
                self.player.state = State::Nil;
            }
        }

        //敵と弾の当たり判定
        for enemy in self.enemy_list.iter_mut() {
            for bullet in self.bullet_list.iter_mut() {
                let enemy_pos = enemy.pos;
                let (x1, y1) = (enemy_pos.x - PLAYER_RADIUS, enemy_pos.y + PLAYER_RADIUS);
                let (x2, y2) = (enemy_pos.x + PLAYER_RADIUS, enemy_pos.y - PLAYER_RADIUS);
                if ((bullet.pos.x > x1) && (bullet.pos.x < x2) &&
                        (bullet.pos.y < y1 + BULLET_RADIUS) &&
                        (bullet.pos.y > y2 - BULLET_RADIUS)) ||
                    ((bullet.pos.x > x1 - BULLET_RADIUS) && (bullet.pos.x < x2 + BULLET_RADIUS) &&
                         (bullet.pos.y < y1) && (bullet.pos.y > y2)) ||
                    ((x1 - bullet.pos.x).powf(2.0) + (y1 - bullet.pos.y).powf(2.0) <
                         BULLET_RADIUS.powf(2.0)) ||
                    ((x2 - bullet.pos.x).powf(2.0) + (y1 - bullet.pos.y).powf(2.0) <
                         BULLET_RADIUS.powf(2.0)) ||
                    ((x2 - bullet.pos.x).powf(2.0) + (y2 - bullet.pos.y).powf(2.0) <
                         BULLET_RADIUS.powf(2.0)) ||
                    ((x1 - bullet.pos.x).powf(2.0) + (y2 - bullet.pos.y).powf(2.0) <
                         BULLET_RADIUS.powf(2.0))
                {
                    enemy.state = State::Nil;
                    bullet.state = State::Nil;
                }
            }
        }

        let (width, height) = self.display.get_framebuffer_dimensions();
        let (width, height) = (width as f32, height as f32);
        let caluculate_extrusion = |Position { x, y }| {
            let x = if x - PLAYER_RADIUS < 0.0 {
                0.0 + PLAYER_RADIUS
            } else if x + PLAYER_RADIUS > width {
                width - PLAYER_RADIUS
            } else {
                x
            };
            let y = if y - PLAYER_RADIUS < 0.0 {
                0.0 + PLAYER_RADIUS
            } else if y + PLAYER_RADIUS > height {
                height - PLAYER_RADIUS
            } else {
                y
            };
            Position { x: x, y: y }
        };

        self.player.pos = caluculate_extrusion(self.player.pos);
        for enemy in self.enemy_list.iter_mut() {
            enemy.pos = caluculate_extrusion(enemy.pos);
        }

        let on_field = |Position { x, y }| {
            0.0 <= x - BULLET_RADIUS && x + BULLET_RADIUS <= width && 0.0 <= y - BULLET_RADIUS &&
                y + BULLET_RADIUS <= height
        };
        self.bullet_list.retain(|ref bullet| {
            (bullet.state != State::Nil) && on_field(bullet.pos)
        });
        self.enemy_list.retain(
            |ref enemy| enemy.state != State::Nil,
        );

    }


    pub fn draw(&self) {
        let mut render = Render::new(&self.display);
        render.clear_color(1.0, 1.0, 1.0, 1.0);
        let player: &Player = &self.player;
        let Position { x, y } = player.pos;
        render.draw_rectangle(Position { x: x, y: y }, PLAYER_RADIUS);
        for enemy in self.enemy_list.iter() {
            let Position { x, y } = enemy.pos;
            render.draw_rectangle(Position { x: x, y: y }, PLAYER_RADIUS);
        }
        for bullet in self.bullet_list.iter() {
            let Position { x, y } = bullet.pos;
            render.draw_circle(Position { x: x, y: y }, BULLET_RADIUS, 1.0, 1.0);
        }
        render.finish();
    }
}
