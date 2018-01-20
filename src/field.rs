use glium::Display;
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};

use object::*;
use render::Render;
use file_reader::AppearLocation;

pub struct Field {
    display: Display,
    pub events_loop: EventsLoop,
    pub player: Player,
    pub bullet_list: Vec<Bullet>,
    pub enemy_list: Vec<Enemy>,
    explode_enemy_list: Vec<Enemy>,
    appear_location_list: Vec<AppearLocation>,
    counter: usize,
}

impl Field {
    pub fn new(width: u32, height: u32) -> Field {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new().with_dimensions(width, height);
        let context = ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        let player = Player {
            pos: Position { x: 70.0, y: 70.0 },
            vector: Vector { x: 0.0, y: 0.0 },
            remain_bullet: 0,
            state: State::Existing,
        };
        let mut enemy_list: Vec<Enemy> = Vec::new();
        let mut bullet_list = Vec::new();
        let mut explode_enemy_list: Vec<Enemy> = Vec::new();
        //enemy_list.push(Enemy {
        //    pos: Position { x: 600.0, y: 400.0 },
        //    vector: Vector {
        //        vec_x: -2.0,
        //        vec_y: 0.0,
        //    },
        //    state: State::Existing,
        //    explode_radius: 0.0,
        //});
        //enemy_list.push(Enemy {
        //    pos: Position { x: 600.0, y: 450.0 },
        //    vector: Vector {
        //        vec_x: -2.0,
        //        vec_y: 0.0,
        //    },
        //    state: State::Existing,
        //    explode_radius: 0.0,
        //});
        //bullet_list.push(Bullet {
        //    pos: Position { x: 100.0, y: 400.0 },
        //    vector: Vector {
        //        vec_x: 1.0,
        //        vec_y: 0.0,
        //    },
        //    state: State::Existing,
        //});
        let mut appear_location_list = AppearLocation::read_list("enemy_appearance.pat");
        appear_location_list.reverse();
        Field {
            display: display,
            events_loop: events_loop,
            player: player,
            bullet_list: bullet_list,
            enemy_list: enemy_list,
            explode_enemy_list: explode_enemy_list,
            appear_location_list: appear_location_list,
            counter: 0,
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
        self.load_enemy_location();
        self.update_enemy_vector();
        self.counter += 1;
    }

    fn load_enemy_location(&mut self) {
        if self.appear_location_list.is_empty() ||
            self.appear_location_list[self.appear_location_list.len() - 1].dt != self.counter
        {
            return;
        } else {
            let (width, height) = self.display.get_framebuffer_dimensions();
            let Position { x, y } = self.appear_location_list.pop().unwrap().pos;
            let mut enemy_pos = Position {
                x: x * (width as f32 / PLAYER_RADIUS),
                y: y * (height as f32 / PLAYER_RADIUS),
            };
            loop {
                self.enemy_list.push(Enemy {
                    pos: enemy_pos,
                    vector: Vector { x: 0.0, y: 0.0 },
                    state: State::Existing,
                    explode_radius: 0.0,
                });
                if self.appear_location_list[self.appear_location_list.len() - 1].dt != 0 {
                    self.counter = 0;
                    break;
                }
                let Position { x, y } = self.appear_location_list.pop().unwrap().pos;
                enemy_pos = Position {
                    x: x * (width as f32 / PLAYER_RADIUS),
                    y: y * (height as f32 / PLAYER_RADIUS),
                };
            }
        }
    }

    fn update_enemy_vector(&mut self) {
        let player_pos = self.player.pos;
        for enemy in self.enemy_list.iter_mut() {
            let enemy_pos = enemy.pos;
            let vec_x = player_pos.x - enemy_pos.x;
            let vec_y = player_pos.y - enemy_pos.y;
            let dir = Vector {
                x: vec_x / vec_x.abs().max(vec_y.abs()),
                y: vec_y / vec_x.abs().max(vec_y.abs()),
            };
            enemy.vector = dir;
        }
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

        for enemy in self.enemy_list.iter_mut() {
            for expl in self.explode_enemy_list.iter_mut() {
                let enemy_pos = enemy.pos;
                let (x1, y1) = (enemy_pos.x - PLAYER_RADIUS, enemy_pos.y + PLAYER_RADIUS);
                let (x2, y2) = (enemy_pos.x + PLAYER_RADIUS, enemy_pos.y - PLAYER_RADIUS);
                let explode_radius = expl.explode_radius;

                if ((expl.pos.x > x1) && (expl.pos.x < x2) &&
                        (expl.pos.y < y1 + expl.explode_radius) &&
                        (expl.pos.y > y2 - expl.explode_radius)) ||
                    ((expl.pos.x > x1 - expl.explode_radius) &&
                         (expl.pos.x < x2 + expl.explode_radius) &&
                         (expl.pos.y < y1) && (expl.pos.y > y2)) ||
                    ((x1 - expl.pos.x).powf(2.0) + (y1 - expl.pos.y).powf(2.0) <
                         expl.explode_radius.powf(2.0)) ||
                    ((x2 - expl.pos.x).powf(2.0) + (y1 - expl.pos.y).powf(2.0) <
                         expl.explode_radius.powf(2.0)) ||
                    ((x2 - expl.pos.x).powf(2.0) + (y2 - expl.pos.y).powf(2.0) <
                         expl.explode_radius.powf(2.0)) ||
                    ((x1 - expl.pos.x).powf(2.0) + (y2 - expl.pos.y).powf(2.0) <
                         expl.explode_radius.powf(2.0))
                {
                    enemy.state = State::Exploded;
                }
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
                    bullet.state = State::Nil;
                    enemy.state = State::Exploded;
                }
            }
        }

        //爆発した敵を爆発リストに追加
        for enemy in self.enemy_list.iter_mut() {
            if enemy.state == State::Exploded {
                self.explode_enemy_list.push(enemy.clone());
            }
        }
        //爆風を広げる
        for expl in self.explode_enemy_list.iter_mut() {
            expl.explode_radius = expl.explode_radius + 2.0;
        }

        //壁の当たり判定と押し出し処理
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
            (bullet.state == State::Existing) && on_field(bullet.pos)
        });
        self.enemy_list.retain(
            |ref enemy| enemy.state == State::Existing,
        );
        self.explode_enemy_list.retain(|ref expl| {
            expl.explode_radius <= MAXIMUM_EXPLODE_RADIUS
        });
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
        for expl in self.explode_enemy_list.iter() {
            let Position { x, y } = expl.pos;
            let explode_radius = expl.explode_radius;
            render.draw_circle(Position { x: x, y: y }, explode_radius, 1.0, 1.0);
        }
        for bullet in self.bullet_list.iter() {
            let Position { x, y } = bullet.pos;
            render.draw_circle(Position { x: x, y: y }, BULLET_RADIUS, 1.0, 1.0);
        }
        render.finish();
    }
}
