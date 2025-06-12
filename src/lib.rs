pub mod strategy;

use crate::strategy::get_point;

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[no_mangle]
pub extern "C" fn getPoint(
    m: i32,
    n: i32,
    top: *const i32,
    board: *const i32,
    last_x: i32,
    last_y: i32,
    no_x: i32,
    no_y: i32,
) -> *mut Point {

    let m = m as usize;
    let n = n as usize;
    let no_x = no_x as usize;
    let no_y = no_y as usize;

    let top_vec: Vec<i32> = unsafe {
        assert!(!top.is_null());
        std::slice::from_raw_parts(top, n).to_vec()
    };

    let board_vec: Vec<Vec<i32>> = unsafe {
        assert!(!board.is_null());
        let flat_board = std::slice::from_raw_parts(board, m * n);
        flat_board
            .chunks(n)
            .map(|row| row.to_vec())
            .collect()
    };

    let (x, y) = get_point(
        m,
        n,
        &top_vec,
        &board_vec,
        last_x,
        last_y,
        no_x,
        no_y,
    );

    let point = Point {
        x,
        y,
    };

    Box::into_raw(Box::new(point))
}

#[no_mangle]
pub extern "C" fn clearPoint(p: *mut Point) {
    if !p.is_null() {
        unsafe {
            let _ = Box::from_raw(p);
        }
    }
}