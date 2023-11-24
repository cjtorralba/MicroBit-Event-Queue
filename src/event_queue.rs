///





extern crate alloc;
use alloc::vec::Vec;
use embedded_alloc::Heap;
use microbit::hal::Timer;
use microbit::hal::pac::SYST;

use crate::events::Event;



/// Basic Queue structure, consists of a Vec which we will use as our "queue"
pub struct Queue<T> {

    /// Main part of the queue, `queue.get(0)` will be the most recently added item.
    /// `queue.get(queue.size())` will be the item last added to the queue
    queue: Vec<T>,
}



/// Contains an enum `Event` with a corresponding timing `u32`
pub struct TimedEvent {
    event: Event,
    timing: u32,
}



struct TimedEventQueue {
    queue: Queue<TimedEvent>,
    rtc: Rtc,

}

impl TimedEventQueue {

}




impl TimedEvent {


    pub fn new(event: Event) -> Self {


        TimedEvent {
            event,
            timing: 0,
        }

    }





}



impl<T> Queue<T> {


    /// Produces a new queue from desired Vector
    pub fn new(queue: Vec<T>) -> Self {
        Queue{
            queue,
        }
    }


    /// Generates a default queue, allocates a new vector
    pub fn default() -> Self {
        Queue{
            queue: Vec::new(),
        }
    }


    /// Adds an item to the front of the queue
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


    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.queue.get(index)
    }


}