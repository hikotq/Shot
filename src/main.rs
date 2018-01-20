#[macro_use]
extern crate glium;
mod render;
mod field;
mod object;
mod file_reader;

use std::{thread, time};
use field::Field;


fn main() {
    use glium::glutin;
    let width = 1000;
    let height = 700;
    let mut field = Field::new(width, height);

    let mut closed = false;
    while !closed {
        field.update();
        field.exe_player_cmd(object::Command::Shot(object::Direction::Right));
        field.exe_player_cmd(object::Command::Move(object::ExtendDirection::RightUp));
        let ten_millis = time::Duration::from_millis(10);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
        field.draw();
        field.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                }
            }
            _ => (),
        });
    }
}
