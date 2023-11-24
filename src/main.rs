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
    pac::{self, interrupt},

    pac::twi0::frequency::FREQUENCY_A,
};


use nrf52833_hal::rtc::{Rtc, RtcInterrupt};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use microbit::hal::timer;
use microbit::pac::Interrupt::RTC1;
use nrf52833_hal::pac::RTC0;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use ssd1306::command::Command;

use crate::event_queue::*;


#[global_allocator]
static HEAP: Heap = Heap::empty();




static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));














#[entry]
fn main() -> ! {


    rtt_init_print!();

    Rtc::new(RTC0::PTR, 1).unwrap();


    /*
        Initializing allocator
     */


    const HEAP_SIZE: usize = 8192;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }





    let board = Board::take().unwrap();
    let mut touch_pin = board.pins.p1_04.into_floating_input();
    let mut timer = timer::Timer::new(board.TIMER0);



    let mut button_a = board.buttons.button_a.into_floating_input();
    let mut button_b = board.buttons.button_b.into_floating_input();

    let rtc = board.RTC0;






    loop {

        if button_a.is_low().unwrap() {
            rprintln!("Button a pressed");
        }



    }
}
