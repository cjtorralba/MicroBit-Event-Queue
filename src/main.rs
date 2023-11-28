#![no_main]
#![no_std]

extern crate alloc;

mod event_queue;
mod events;

use core::cell::RefCell;
use panic_rtt_target as _;
use core::mem::MaybeUninit;
use core::ops::BitAnd;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::{entry};
use embedded_alloc::Heap;
use microbit::{
    board::Board,
    hal::{
        gpiote::Gpiote,
        twim,
        prelude::*,
    },

};



use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use microbit::hal::gpio::Level;
use microbit::hal::timer;
use microbit::hal::Rtc;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use ssd1306::command::Command;

use crate::event_queue::*;
use crate::events::{Button, Event};


#[global_allocator]
static HEAP: Heap = Heap::empty();




static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));














#[entry]
fn main() -> ! {


    rtt_init_print!();



    /*
        Initializing allocator
     */


    const HEAP_SIZE: usize = 8192;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }





    let board = Board::take().unwrap();
    let mut touch_pin = board.pins.p1_04.into_floating_input();
    let mut timer = timer::Timer::new(board.TIMER0);



    let mut button_a = board.buttons.button_a.into_push_pull_output(Level::Low);
    let mut button_b = board.buttons.button_b.into_floating_input();

    let rtc = Rtc::new(board.RTC0, 33).unwrap();

    let mut event_queue = TimedEventQueue::new(rtc);







    loop {

        if button_a.is_set_low().unwrap(){
            rprintln!("Button a low");
        }

        if button_a.is_set_high().unwrap(){
            rprintln!("Button a high");
        }


    }
}
