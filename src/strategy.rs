pub fn get_point(
    row: usize,
    col: usize,
    top: &Vec<i32>,
    board: &Vec<Vec<i32>>,
    last_x: i32,
    last_y: i32,
    nox: usize,
    noy: usize,
) -> (i32, i32) {
    let mut x: i32 = -1;
    let mut y: i32 = -1;
    for i in 0..col {
        if top[i] > 0 {
            x = top[i] - 1;
            y = i as i32;
            break;
        }
    }
    eprintln!("get_point returns ({}, {})", x, y); // Example of printing debug messages to stderr.
    (x, y)
}
