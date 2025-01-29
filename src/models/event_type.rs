use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EventType {
    Socket = 0,
    Process = 1,
    HWusage = 2,
}

impl EventType {
    pub fn from_u8(data: u8) -> Self {
        match data {
            0 => EventType::Socket,
            1 => EventType::Process,
            2 => EventType::HWusage,
            3_u8..=u8::MAX => panic!("invalid event"),
        }
    }

    pub fn get_value(&self) -> u8 {
        match self {
            EventType::Socket => 0,
            EventType::Process => 1,
            EventType::HWusage => 2,
        }
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}
