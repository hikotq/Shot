#[macro_use]
extern crate glium;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate rand;
mod render;
mod field;
mod object;
mod file_reader;

use std::collections::HashMap;
use std::{thread, time};
use std::fs::{File, OpenOptions};
use std::io::Write;
use rand::Rng;
use object::Command;
use field::Field;

static ALPHA: f64 = 0.1;
static DISCOUNT_RATE: f64 = 0.92;
static EPISILON: f64 = 0.3;

type State = u64;

fn command_select(
    q_table: &HashMap<State, HashMap<Command, f64>>,
    state: State,
    epsilon: f64,
) -> Command {
    let random_num: usize = rand::thread_rng().gen_range(0, 100);
    if random_num < (epsilon * 100.0) as usize || !q_table.contains_key(&state) ||
        q_table.get(&state).unwrap().len() == 0
    {
        let random_command_num = rand::thread_rng().gen_range(0, 13);
        *Command::iterator().nth(random_command_num).unwrap()
    } else {
        let mut sorted_command_qval: Vec<(&Command, &f64)> =
            q_table.get(&state).unwrap().iter().collect();
        sorted_command_qval.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let cmd = match sorted_command_qval.first() {
            Some(&(&cmd, q)) => {
                //if *q as usize != 0 {
                //    println!("{}", q);
                //}
                cmd}, 
            None => Command::Stay, 
        };
        cmd
    }
}

fn learn() {
    use glium::glutin;
    let width = 400;
    let height = 400;
    let mut field = Field::new(width, height);
    let max_cicle = 3;
    let s_start = field.get_hash();

    let mut closed = false;
    let mut q_table: HashMap<State, HashMap<Command, f64>> = match File::open("q_table.bin") {
        Ok(file) => bincode::deserialize_from(file).unwrap(),
        Err(_) => HashMap::new(),
    };
    for cicle in 0..max_cicle {
        field.reset();
        let mut state: State = field.get_hash();
        while !(field.game_end || closed) {
            q_table.entry(state).or_insert(HashMap::new());
            let command = command_select(&q_table, state, EPISILON);
            let (next_state, reward) = field.next_state(state, command);
            let q_value = q_table.get(&state).unwrap().get(&command).unwrap_or(&0.0) +
                ALPHA *
                    (reward +
                         DISCOUNT_RATE *
                             Command::iterator()
                                 .map(|command| q_table.get(&state).unwrap().get(&command))
                                 .map(|value| value.unwrap_or(&0.0))
                                 .fold(0.0 / 0.0, |m, v| v.max(m)) -
                         q_table.get(&state).unwrap().get(&command).unwrap_or(&0.0));
            q_table.get_mut(&state).unwrap().insert(command, q_value);
            state = next_state;
            
            //let ten_millis = time::Duration::from_millis(10);
            //let now = time::Instant::now();
            //thread::sleep(ten_millis);
            //field.draw();
            //field.events_loop.poll_events(|event| match event {
            //    glutin::Event::WindowEvent { event, .. } => {
            //        match event {
            //            glutin::WindowEvent::Closed => closed = true,
            //            _ => (),
            //      }
            //    }
            //    _ => (),
            //});
        }
        let mut file = match OpenOptions::new().write(true).append(true).open(
            "score.csv",
        ) {
            Ok(file) => file,
            Err(_) => File::create("score.csv").unwrap(),
        };
        file.write_fmt(format_args!("{},", field.score));
    }
    let mut file = File::create("q_table.bin").unwrap();
    bincode::serialize_into(&mut file, &q_table).unwrap();
}

fn draw() {
    use glium::glutin;
    let mut file = File::open("q_table.bin").unwrap();

    let width = 400;
    let height = 80;
    let mut field = Field::new(width, height);
    let mut q_table: HashMap<State, HashMap<Command, f64>> = bincode::deserialize_from(file)
        .unwrap();
    let mut closed = false;
    while !(closed || field.game_end) {
        let state = field.get_hash();
        let cmd = command_select(&q_table, state, 0.1);
        field.exec_player_cmd(cmd);
        field.update();
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

fn main() {
    draw();
}
