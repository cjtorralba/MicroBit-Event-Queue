


/// Enum that accounts for button presses, including buttonA, buttonB and touch logo
pub enum Button {
    ButtonA,
    ButtonB,
    TouchLogo,
}




/// Enum to account for basic events, including button press and release, microphone input, speaker output
pub enum Event {
    ButtonPress(Button),
    ButtonRelease(Button),
}


