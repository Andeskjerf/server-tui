use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum EventType {
    SOCKET = 0,
    PROCESS = 1,
}

impl EventType {
    pub fn from_u8(data: u8) -> Self {
        match data {
            0 => EventType::SOCKET,
            1 => EventType::PROCESS,
            2_u8..=u8::MAX => panic!("invalid event"),
        }
    }

    pub fn get_value(&self) -> u8 {
        match self {
            EventType::SOCKET => 0,
            EventType::PROCESS => 1,
        }
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}
