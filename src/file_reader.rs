use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use object::Position;

pub struct AppearLocation {
    pub dt: usize,
    pub pos: Position,
}

impl AppearLocation {
    pub fn read_list(list_file_name: &str) -> Vec<AppearLocation> {
        let path = Path::new(list_file_name);
        let display = path.display();

        // pathを読み込み専用モードで開く。これは`io::Result<File>`を返す。
        let mut file = match File::open(&path) {
            // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
            Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };

        // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
        let mut s = String::new();
        let mut lines = Vec::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
            Ok(_) => lines = s.split('\n').collect(),
        };
        let mut appear_location_list = Vec::new();
        for line in lines {
            if line.len() < 3 {
                break;
            }
            let line: Vec<&str> = line.split(',').collect();
            let dt: usize = line[0].parse().unwrap();
            let x: f32 = line[1].parse().unwrap();
            let y: f32 = line[2].parse().unwrap();
            let appear_location = AppearLocation {
                dt: dt,
                pos: Position { x: x, y: y },
            };
            appear_location_list.push(appear_location);
        }
        appear_location_list
    }
}
