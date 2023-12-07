### MicroBit Event Queue

Authors: Christian Torralba, Barton Massey

#### Brief Idea/Purpose

A quick intro to this to just keep track of user interaction with the microbit. This means button presses, microphone pickup etc.
The program will keep track of the event and when it occurred using a timer.

Currently, there are 3 events you can keep track of, button a, b and the touchpad.


#### Thought process:
Learning to work with lower level things like events and interrupts was a real challenge for me. Especially the interrupt handler.
I had been so used to not having global variables, it felt strange at first. The same goes for working with peripherals, the buttons and microphone.

It was very fun to get to work with the `pac` and the `hal` crate, they provided me with much needed into about all the different 
peripherals and hardware I could use.


#### Future use:
I see this integrating decently well with other time-based projects with minimal changes. I plan on making something akin to 
a chess clock. Or any other things that have to use a timer.

#### Overview
While a tad bit overwhelming, once I got semi-used to the program flow and how you can use each peripheral, as well as events and channels,
things started coming together. There is still plenty I want to do with this project but as of now it is a small demo.