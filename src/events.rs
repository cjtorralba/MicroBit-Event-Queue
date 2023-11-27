/// Enum that accounts for button presses, including buttonA, buttonB and touch logo
#[derive(Clone, Debug)]
pub enum Button {
    ButtonA,
    ButtonB,
    TouchLogo,
}


/// Enum to account for basic events, including button press and release, microphone input, speaker output
#[derive(Clone, Debug)]
pub enum Event {
    ButtonPress(Button),
    ButtonRelease(Button),
}


