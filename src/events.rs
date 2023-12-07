use core::fmt;
use core::fmt::Formatter;

/// Enum that accounts for button presses, including buttonA, buttonB and touch logo
#[derive(Clone, Debug)]
pub enum Button {
    ButtonA,
    ButtonB,
    TouchLogo,
}

impl fmt::Display for Button {
    /// Basic display implementation for [Button] enum
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Button::ButtonA => {
                write!(f, "ButtonA")
            }
            Button::ButtonB => {
                write!(f, "ButtonB")
            }
            Button::TouchLogo => {
                write!(f, "Touchpad")
            }
        }
    }
}

/// Enum to account for basic events, including button press and release, microphone input, speaker output
#[derive(Clone, Debug)]
pub enum Event {
    ButtonPress(Button),
    ButtonRelease(Button),
    MicroPhoneInput,
}

impl fmt::Display for Event {
    /// Basic display implementation for the [Event] enum
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Event::ButtonPress(button) => {
                write!(f, "{}", button)
            }
            Event::ButtonRelease(button) => {
                write!(f, "{}", button)
            }
            Event::MicroPhoneInput => {
                write!(f, "MicroPhoneInput")
            }
        }
    }
}
