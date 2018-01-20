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
