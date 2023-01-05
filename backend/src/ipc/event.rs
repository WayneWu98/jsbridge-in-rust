use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Event {
    ThemeChanged,
    SystemLog,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::ThemeChanged => write!(f, "ThemeChanged"),
            Event::SystemLog => write!(f, "SystemLog"),
        }
    }
}
