### MicroBit Event Queue

Authors: Christian Torralba, Barton Massey

#### Brief Idea/Purpose

A quick intro to this to just keep track of user interaction with the microbit. This means button presses, microphone pickup etc.
The program will keep track of the event and when it occurred using a timer.

#### Example usage (subject to change)

```rust

/// Aquire board, we will need this to reach our RTC
let board = Board::take().unwrap();


/// Create a new RTC of your choice, with the scalar of your choice
let scalar: u32 = 33;
let rtc = Rtc::new(board.RTC0, scalar).unwrap();


/// Create event queue
let mut event_queue = TimedEventQueue::new(rtc);


/// Begin adding your events
loop {

// pseudocode
 if buttonAPressed {
    event_queue.add_event(Event::ButtonPress(Button::ButtonA));
    }

}

```