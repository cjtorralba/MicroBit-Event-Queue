extern crate alloc;

use alloc::vec::Vec;

use microbit::hal::{Rtc, rtc};

use crate::events::Event;

/// Basic Queue structure, consists of a Vec which we will use as our "queue"
#[derive(Clone, Debug)]
pub struct Queue<T> {

    /// Main part of the queue, `queue.get(0)` will be the most recently added item.
    /// `queue.get(queue.size())` will be the item last added to the queue
    queue: Vec<T>,
}



/// Contains an enum `Event` with a corresponding timing `u32`
#[derive(Clone, Debug)]
pub struct TimedEvent {
    event: Event,
    timing: u32,
}


/// This struct contains a [Queue] of [TimedEvents](TimedEvent), as well as an RTC<T> counter where T
/// is an [rtc::Instance]
pub struct TimedEventQueue<T> {
    queue: Queue<TimedEvent>,
    rtc: Rtc<T>,
}




impl<T: rtc::Instance> TimedEventQueue<T> {


    /// Creates a new [TimedEVentQueue] and both clears and enables the RTC counter
    pub fn new(rtc: Rtc<T>) -> Self {

        // Clear current counter value, need to reset since we are initializing new TimedEventQueue
        rtc.clear_counter();

        // Starting counter back up from 0
        rtc.enable_counter();
        TimedEventQueue {
            queue: Queue::default(),
            rtc,
        }
    }


    /// Adds an [Event] to the queue.
    /// Uses current time on rtc via get_counter method in [nrf_hal_common::rtc]
    pub fn add_event(&mut self, event: Event) {
        let timing = self.rtc.get_counter();
        self.queue.enqueue(TimedEvent::new(event.clone(), timing));
    }



    /// ### Returns
    /// - [Option]<&[TimedEvent]>
    ///
    /// ### Arguments:
    /// - index: [usize]
    ///     - Index you with to get the timed event from in the queue
    pub fn get_event(&self, index: usize) -> Option<&TimedEvent> {
        self.queue.get(index)
    }



}




impl TimedEvent {

    /// ### Returns:
    /// - [Self](TimedEvent)
    ///
    /// ### Arguments:
    /// - event: [Event]
    /// - timing: [u32]
    pub fn new(event: Event, timing: u32) -> Self {
        TimedEvent {
            event,
            timing,
        }
    }
}



impl<T> Queue<T> {


    /// Produces a new [Queue] from desired [vector]<T> where T is a type parameter
    pub fn new(queue: Vec<T>) -> Self {
        Queue{
            queue,
        }
    }


    /// Generates a default [Queue], allocates a new [vector](Vec)
    pub fn default() -> Self {
        Queue{
            queue: Vec::new(),
        }
    }


    /// Adds an item to the front of the queue
    ///
    /// ### Arguments:
    ///     item: T
    ///
    /// ### Example:
    /// ```rs
    /// let q = Queue::default();
    /// q.enqueue(1);
    ///
    /// assert_eq!(q, [1]);
    ///
    /// q.enqueue(2);
    ///
    /// assert_eq!(q, [1, 2]);
    /// ```
    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }


    /// Dequeues an Item from the queue, will remove first item added
    ///
    /// ### Example:
    /// ```rs
    /// let queue = Queue::default();
    ///
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.enqueue(3);
    ///
    /// assert_eq!(queue, [1, 2, 3]);
    ///
    /// q.dequeue();
    ///
    /// assert_eq!(q, [2, 3]);
    /// ```
    pub fn dequeue(&mut self) {
        self.queue.remove(0);
    }




    /// Returns the `size` of the queue
    ///
    /// ### Returns:
    /// -    usize
    ///
    /// ### Example:
    /// ```rs
    /// let q = Queue::default();
    /// q.enqueue(1);
    /// q.enqueue(2);
    /// q.enqueue(3);
    ///
    /// assert_eq!(q.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        self.queue.len()
    }


    /// ### Returns:
    /// - `Option<&T>`
    ///
    /// Returns first item in the queue
    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }


    /// ### Returns:
    /// - `Option<&T>`
    ///
    /// ### Arguments:
    /// - index: usize
    ///
    /// Returns given item based off index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.queue.get(index)
    }


}