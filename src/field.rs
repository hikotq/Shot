use glium::Display;
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};

use object::*;
use render::{Color, Render};
use file_reader::AppearLocation;

pub struct Field {
    display: Display,
    pub events_loop: EventsLoop,
    pub player: Player,
    pub bullet_list: Vec<Bullet>,
    pub enemy_list: Vec<Enemy>,
    explosion_list: Vec<Explosion>,
    appear_location_list: Vec<AppearLocation>,
    appearance_counter: usize,
    pub score: u64,
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
            remain_bullet: MAXIMUM_BULLET,
            state: State::Existing,
            bullet_timer: 0,
        };
        let mut enemy_list: Vec<Enemy> = Vec::new();
        let mut bullet_list = Vec::new();
        let mut explosion_list: Vec<Explosion> = Vec::new();
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
            explosion_list: explosion_list,
            appear_location_list: appear_location_list,
            appearance_counter: 0,
            score: 0,
        }
    }

    pub fn update(&mut self) {
        self.player.update();
        for enemy in self.enemy_list.iter_mut() {
            enemy.update();
        }
        for bullet in self.bullet_list.iter_mut() {
            bullet.update();
        }
        self.detect_collision();
        self.load_enemy_location();
        self.update_enemy_vector();
        self.appearance_counter += 1;
    }

    fn load_enemy_location(&mut self) {
        if self.appear_location_list.is_empty() ||
            self.appear_location_list[self.appear_location_list.len() - 1].dt !=
                self.appearance_counter
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
                });
                if self.appear_location_list.is_empty() ||
                    self.appear_location_list[self.appear_location_list.len() - 1].dt != 0
                {
                    self.appearance_counter = 0;
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

        let mut explosion_tmp_buffer = Vec::new();
        //爆発の当たり判定
        for enemy in self.enemy_list.iter_mut() {
            for expl in self.explosion_list.iter_mut() {
                let enemy_pos = enemy.pos;
                let (x1, y1) = (enemy_pos.x - PLAYER_RADIUS, enemy_pos.y + PLAYER_RADIUS);
                let (x2, y2) = (enemy_pos.x + PLAYER_RADIUS, enemy_pos.y - PLAYER_RADIUS);
                let explosion_radius = expl.radius;

                if ((expl.pos.x > x1) && (expl.pos.x < x2) && (expl.pos.y < y1 + expl.radius) &&
                        (expl.pos.y > y2 - expl.radius)) ||
                    ((expl.pos.x > x1 - expl.radius) && (expl.pos.x < x2 + expl.radius) &&
                         (expl.pos.y < y1) && (expl.pos.y > y2)) ||
                    ((x1 - expl.pos.x).powf(2.0) + (y1 - expl.pos.y).powf(2.0) <
                         expl.radius.powf(2.0)) ||
                    ((x2 - expl.pos.x).powf(2.0) + (y1 - expl.pos.y).powf(2.0) <
                         expl.radius.powf(2.0)) ||
                    ((x2 - expl.pos.x).powf(2.0) + (y2 - expl.pos.y).powf(2.0) <
                         expl.radius.powf(2.0)) ||
                    ((x1 - expl.pos.x).powf(2.0) + (y2 - expl.pos.y).powf(2.0) <
                         expl.radius.powf(2.0))
                {
                    enemy.state = State::Nil;
                    explosion_tmp_buffer.push(Explosion {
                        pos: enemy_pos,
                        radius: 0.0,
                        chain: expl.chain + 1,
                    });
                    self.score += KILLING_POINT * (expl.chain + 1);
                    println!("chain!");
                }
            }
        }

        //敵と弾の当たり判定
        for enemy in self.enemy_list.iter_mut() {
            for bullet in self.bullet_list.iter_mut() {
                let enemy_pos = enemy.pos;
                let (x1, y1) = (enemy_pos.x - PLAYER_RADIUS, enemy_pos.y + PLAYER_RADIUS);
                let (x2, y2) = (enemy_pos.x + PLAYER_RADIUS, enemy_pos.y - PLAYER_RADIUS);
                if enemy.state == State::Existing &&
                    ((bullet.pos.x > x1) && (bullet.pos.x < x2) &&
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
                    enemy.state = State::Nil;
                    explosion_tmp_buffer.push(Explosion {
                        pos: enemy_pos,
                        radius: 0.0,
                        chain: 1,
                    });
                    self.score += KILLING_POINT;
                }
            }
        }

        //爆発を追加
        self.explosion_list.append(&mut explosion_tmp_buffer);

        //爆発を広げる
        for expl in self.explosion_list.iter_mut() {
            expl.radius = expl.radius + 2.0;
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
        self.explosion_list.retain(|ref expl| {
            expl.radius <= MAXIMUM_EXPLODE_RADIUS
        });
    }

    pub fn exec_player_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Shot(dir) => self.shot(dir),
            Command::Move(exdir) => self.player_move(exdir),
            Command::Stay => (),
        };
    }

    pub fn shot(&mut self, dir: Direction) {
        if self.player.remain_bullet == 0 {
            return;
        }
        self.player.remain_bullet -= 1;

        let vec = match dir {
            Direction::Left => {
                Vector {
                    x: -BULLET_SPEED,
                    y: 0.0,
                }
            }
            Direction::Right => {
                Vector {
                    x: BULLET_SPEED,
                    y: 0.0,
                }
            }
            Direction::Up => Vector {
                x: 0.0,
                y: BULLET_SPEED,
            },
            Direction::Down => Vector {
                x: 0.0,
                y: -BULLET_SPEED,
            },
        };

        let player_pos = self.player.pos;
        let bullet = Bullet {
            pos: Position {
                x: player_pos.x +
                    if vec.x != 0.0 {
                        (vec.x / vec.x.abs())
                    } else {
                        0.0
                    } * PLAYER_RADIUS,
                y: player_pos.y +
                    if vec.y != 0.0 {
                        (vec.y / vec.y.abs())
                    } else {
                        0.0
                    } * PLAYER_RADIUS,
            },
            vector: vec,
            state: State::Existing,
        };
        self.bullet_list.push(bullet);
    }

    pub fn player_move(&mut self, dir: ExtendDirection) {
        let vec = match dir {
            ExtendDirection::Left => Vector {
                x: -PLAYER_SPEED,
                y: 0.0,
            },
            ExtendDirection::Right => Vector {
                x: PLAYER_SPEED,
                y: 0.0,
            },
            ExtendDirection::Up => Vector {
                x: 0.0,
                y: PLAYER_SPEED,
            },
            ExtendDirection::Down => Vector {
                x: 0.0,
                y: -PLAYER_SPEED,
            }, 
            ExtendDirection::LeftUp => Vector {
                x: -PLAYER_SPEED,
                y: PLAYER_SPEED,
            },
            ExtendDirection::RightUp => Vector {
                x: PLAYER_SPEED,
                y: PLAYER_SPEED,
            },
            ExtendDirection::LeftDown => Vector {
                x: -PLAYER_SPEED,
                y: -PLAYER_SPEED,
            },
            ExtendDirection::RightDown => Vector {
                x: PLAYER_SPEED,
                y: -PLAYER_SPEED,
            },
        };
        self.player.vector = vec;
    }

    pub fn draw(&self) {
        let mut render = Render::new(&self.display);
        render.clear_color(1.0, 1.0, 1.0, 1.0);
        let player: &Player = &self.player;
        render.draw_rectangle(
            Position {
                x: player.pos.x,
                y: player.pos.y,
            },
            PLAYER_RADIUS,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                alpha: 1.0,
            },
        );

        for enemy in self.enemy_list.iter() {
            let Position { x, y } = enemy.pos;
            render.draw_rectangle(
                Position { x: x, y: y },
                PLAYER_RADIUS,
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    alpha: 1.0,
                },
            );
        }
        for expl in self.explosion_list.iter() {
            let Position { x, y } = expl.pos;
            let explode_radius = expl.radius;
            render.draw_circle(
                Position { x: x, y: y },
                explode_radius,
                1.0,
                1.0,
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    alpha: 1.0,
                },
            );
        }
        for bullet in self.bullet_list.iter() {
            let Position { x, y } = bullet.pos;
            render.draw_circle(
                Position { x: x, y: y },
                BULLET_RADIUS,
                1.0,
                1.0,
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    alpha: 1.0,
                },
            )
        }
        render.finish();
    }
}
