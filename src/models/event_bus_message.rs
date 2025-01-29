use super::event_type::EventType;

const MESSAGE_LENGTH: usize = 3;

#[derive(Clone)]
pub struct EventBusMessage {
    title: String,
    description: String,
    event_type: EventType,
    timestamp: i64,
}

impl EventBusMessage {
    pub fn new(title: &str, description: &str, event_type: EventType) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            event_type,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        let binding = String::from_utf8(data).expect("unable to decode data");
        let decoded = binding.split(',').collect::<Vec<&str>>();

        if decoded.len() != MESSAGE_LENGTH {
            panic!("invalid message, got {binding}");
        }

        let event_type = match decoded[1].parse::<u8>() {
            Ok(res) => res,
            Err(_) => panic!("unable to parse EventType, got {}", decoded[1]),
        };
        Self {
            title: String::from(decoded[0]),
            description: String::from(decoded[2]),
            event_type: EventType::from_u8(event_type),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn format(&self) -> String {
        format!("{},{},{}", self.title, self.event_type, self.description)
    }

    pub fn format_bytes(&self) -> Vec<u8> {
        self.format().into_bytes().to_vec()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn ts(&self) -> i64 {
        self.timestamp
    }
}
