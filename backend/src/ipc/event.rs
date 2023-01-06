use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Event {
    ThemeChanged,
    SystemLog,
    NetworkChanged,
    CPUChanged,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::ThemeChanged => write!(f, "ThemeChanged"),
            Event::SystemLog => write!(f, "SystemLog"),
            Event::NetworkChanged => write!(f, "NetworkChanged"),
            Event::CPUChanged => write!(f, "CPUChanged"),
        }
    }
}
