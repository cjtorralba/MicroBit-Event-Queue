#![no_main]
#![no_std]

extern crate alloc;

mod event_queue;
mod events;

use panic_rtt_target as _;

use rtt_target::rtt_init_print;
use cortex_m_rt::{entry};
use embedded_alloc::Heap;
use microbit::{
    board::Board,
    hal::{
        twim,
        prelude::*,
    },

    pac::twi0::frequency::FREQUENCY_A,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use microbit::hal::timer;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use ssd1306::command::Command;

use crate::event_queue::*;


#[global_allocator]
static HEAP: Heap = Heap::empty();



#[entry]
fn main() -> ! {


    rtt_init_print!();

    /*
        Initializing allocator
     */

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 8192;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }





    let board = Board::take().unwrap();
    let mut touch_pin = board.pins.p1_04.into_floating_input();
    let mut timer = timer::Timer::new(board.TIMER0);

    let mut event_queue = EventQueue::default();



    loop {




    }
}