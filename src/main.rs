#![no_std]
#![no_main]

extern crate panic_halt;

use microbit as _;
use microbit::{
    board::Board,
    hal::prelude::{OutputPin, PinState},
};

#[cortex_m_rt::entry]
fn start_here() -> ! {
    let mut board = Board::take().unwrap();
    board.display_pins.col3.set_state(PinState::Low).unwrap();
    board.display_pins.row3.set_state(PinState::High).unwrap();
    loop {}
}

