#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
//use panic_halt as _;

use panic_rtt_target as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

use microbit::{board::Board, display::blocking::Display, hal::prelude::*};

const Y_MAX: usize = 4;
const X_MAX: usize = 4;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World from microbit v2");
    let mut matrix = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut board = Board::take().unwrap();

    let mut timer = microbit::hal::Timer::new(board.TIMER0);
    // create the Display
    let mut display = Display::new(board.display_pins);

    let mut x = 0;
    let mut y = 0;
    let mut x_iter = 1;
    let mut y_iter = 0;
    loop {
        let old_x: i32 = x;
        let old_y: i32 = y;

        display.show(&mut timer, matrix, 128);
        display.clear();
        timer.delay_ms(250u32);
        match (x as usize, y as usize) {
            (0, 0) => {
                x_iter = 1;
                y_iter = 0
            }
            (X_MAX, 0) => {
                x_iter = 0;
                y_iter = 1
            }
            (X_MAX, Y_MAX) => {
                x_iter = -1;
                y_iter = 0
            }
            (0, Y_MAX) => {
                x_iter = 0;
                y_iter = -1
            }
            (_, _) => {}
        };
        x += x_iter;
        y += y_iter;

        matrix[old_y as usize][old_x as usize] = 0;
        matrix[y as usize][x as usize] = 1;
    }
}
