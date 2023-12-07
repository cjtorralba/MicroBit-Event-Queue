#![no_main]
#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use core::any::Any;
use panic_rtt_target as _;

use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::ops::BitAnd;

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

// Global gpio
static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));


// Global event queue so the interrupt handler can access it
static EVENT_QUEUE: Mutex<RefCell<Option<TimedEventQueue<RTC0>>>> = Mutex::new(RefCell::new(None));



#[entry]
fn main() -> ! {

    rtt_init_print!();



    const HEAP_SIZE: usize = 8192;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }



    let board = Board::take().unwrap();

    let gpiote = Gpiote::new(board.GPIOTE);


    // Bind Button A to channel0
    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board.buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();


    // Bind button B to channel1
    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board.buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();


    // Microphone input
    let channel2 = gpiote.channel2();
    channel2.input_pin(&board.microphone_pins.mic_in.degrade())
        .enable_interrupt();
    channel2.reset_events();

    let channel3 = gpiote.channel3();
    channel3.input_pin(&board.pins.p1_04.into_floating_input().degrade())
        .hi_to_lo()
        .enable_interrupt();



    // Make instance of RTC
    let rtc = Rtc::new(board.RTC0, 33).unwrap();
    // Pass RTC into EventQueue
    let event_queue = TimedEventQueue::new(rtc);


    cortex_m::interrupt::free(move |cs| {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);
        *EVENT_QUEUE.borrow(cs).borrow_mut() = Some(event_queue);
    });


    loop {

        asm::wfi();
    }
}



#[interrupt]
fn GPIOTE() {
    rprintln!("Interrupt time!");
    cortex_m::interrupt::free(|cs|{
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {

            let button_a_pressed = gpiote.channel0().is_event_triggered();
            let button_b_pressed = gpiote.channel1().is_event_triggered();
            let microphone_triggered = gpiote.channel2().is_event_triggered();
            let touch_pad_triggered = gpiote.channel3().is_event_triggered();
            cortex_m::interrupt::free(|cs| {
                if let Some(mut event) = EVENT_QUEUE.borrow(cs).borrow_mut().as_mut() {
                    if button_a_pressed {
                       event.add_event(Event::ButtonPress(Button::ButtonA));
                        rprintln!("{}", event.get_most_recent_event().unwrap());
                    }

                    if button_b_pressed {
                        event.add_event(Event::ButtonPress(Button::ButtonB));
                        rprintln!("{}", event.get_most_recent_event().unwrap());
                    }

                    if microphone_triggered {
                        event.add_event(Event::MicroPhoneInput);
                        rprintln!("{}", event.get_most_recent_event().unwrap());
                    }

                    if touch_pad_triggered {
                        event.add_event(Event::ButtonPress(Button::TouchLogo));
                        rprintln!("{}", event.get_most_recent_event().unwrap());
                    }
                }
            });



            // Reset all channel events
            gpiote.reset_events();
        }
    });
}
