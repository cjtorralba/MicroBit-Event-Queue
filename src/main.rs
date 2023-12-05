#![no_main]
#![no_std]

extern crate alloc;
use panic_rtt_target as _;

use core::cell::RefCell;
use core::mem::MaybeUninit;

use cortex_m::{
    asm,
    interrupt::Mutex,
};

use cortex_m_rt::entry;

use embedded_alloc::Heap;

use embedded_graphics::{
    prelude::*,
};
use microbit::
{
    board::Board,
    hal::{
        prelude::*,
        gpiote::Gpiote,
        Rtc,
    },
    pac::{
        self,
        interrupt,
        RTC0,
    },
};
use rtt_target::rtt_init_print;
use rtt_target::rprintln;
use ssd1306::{prelude::*};

use crate::event_queue::*;
use crate::events::*;

mod event_queue;
mod events;

#[global_allocator]
static HEAP: Heap = Heap::empty();




static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

static EVENT_QUEUE: Mutex<RefCell<Option<TimedEventQueue<RTC0>>>> = Mutex::new(RefCell::new(None));



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




    let gpiote = Gpiote::new(board.GPIOTE);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board.buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();
    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board.buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();


    let rtc = Rtc::new(board.RTC0, 33).unwrap();
    let mut event_queue = TimedEventQueue::new(rtc);

    cortex_m::interrupt::free(move |cs| {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);
        *EVENT_QUEUE.borrow(cs).borrow_mut() = Some(event_queue);
    });





    loop {

        rprintln!("Waiting for interrupt");
        asm::wfi();
        rprintln!("Got interrupted!");
    }
}



#[interrupt]
fn GPIOTE() {


    rprintln!("Interrupt time!");
    cortex_m::interrupt::free(|cs|{
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let buttonAPressed = gpiote.channel0().is_event_triggered();
            let buttonBPressed = gpiote.channel1().is_event_triggered();

            match (buttonAPressed, buttonBPressed) {
                (false, false) => {

                },
                (true, false) => {

                    cortex_m::interrupt::free(|cs| {
                        if let Some(mut event) = EVENT_QUEUE.borrow(cs).borrow_mut().as_mut() {
                            event.add_event(Event::ButtonPress(Button::ButtonA));
                        } else {
                            rprintln!("Could not add event");
                        }
                    });

                },
                (false, true) => {
                    if let Some(event) = EVENT_QUEUE.borrow(cs).borrow().as_ref() {
                        rprintln!("Button a pressed at: {} time", event.peek().unwrap().timing);
                    }
                },
                (true, true) => {}
            }
            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
        }
    });

}
