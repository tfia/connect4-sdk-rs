pub mod strategy;

use std::io::{stdin, stdout, Write};

use strategy::get_point;

/* Begin main: DO NOT EDIT */
fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let config: Vec<usize> = buffer
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let row = config[0];
    let col = config[1];
    let nox = config[2];
    let noy = config[3];
    let mut board: Vec<Vec<i32>> = vec![vec![0; col]; row];
    let mut top: Vec<i32> = vec![0; col];
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let message: Vec<i32> = buffer
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let last_x = message[0];
        let last_y = message[1];
        for j in 0..col {
            top[j] = message[2 + j];
        }
        for i in 0..row {
            for j in 0..col {
                board[i][j] = message[2 + col + i * col + j];
            }
        }
        let (x, y) = get_point(row, col, &top, &board, last_x, last_y, nox, noy);
        let msg = format!("{} {}", x, y);
        let len = msg.len() as u32;
        print!(
            "{}{}{}{}{}",
            char::from((len >> 24) as u8),
            char::from((len >> 16) as u8),
            char::from((len >> 8) as u8),
            char::from(len as u8),
            msg
        );
        stdout().flush()?;
    }
}
/* End main: DO NOT EDIT */