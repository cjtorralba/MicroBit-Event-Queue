///





extern crate alloc;
use alloc::vec::Vec;
use embedded_alloc::Heap;
use microbit::hal::Timer;

use crate::events::Event;



/// Basic Queue structure, consists of a Vec which we will use as our "queue"
pub struct Queue<T> {

    /// Main part of the queue, `queue.get(0)` will be the most recently added item.
    /// `queue.get(queue.size())` will be the item last added to the queue
    queue: Vec<T>,
}


pub struct EventQueue<T, U> {

    /// Queue of events FIFO
    events: Queue<T>,

    /// time unit corresponding to the time of each event,
    timing: Queue<U>,
}




impl EventQueue<Event, u32> {


    /// Default constructor for EventQueue struct,
    /// Calls `default` constructor for the Queue struct
    pub fn default() -> Self {
        EventQueue{
            events: Queue::default(),
            timing: Queue::default(),
        }
    }


    ///
    pub fn new(events: Queue<Event>, timing: Queue<u32>) -> Self {
        EventQueue{
            events,
            timing,
        }
    }


    pub fn enqueue(&mut self, event: Event, time: u32) {
        self.events.enqueue(event);
        self.timing.enqueue(time);
    }

    pub fn dequeue(&mut self) {
        self.events.dequeue();
        self.timing.dequeue();
    }



    /// Returns the set of `Event` and `Timing` for corresponding index
    pub fn get(&self, index: usize) -> Option<(&Event, &u32)> {
        let event_option: &Option<&Event>= &self.events.get(index);
        let timing_option: &Option<&u32> = &self.timing.get(index);

        if event_option.is_some() && timing_option.is_some() {
            Some((event_option.unwrap(), timing_option.unwrap()))
        } else {
            None
        }
    }



}



impl<T> Queue<T> {

    pub fn new(queue: Vec<T>) -> Self {
        Queue{
            queue,
        }
    }

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