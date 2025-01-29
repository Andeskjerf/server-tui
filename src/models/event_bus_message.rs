use std::collections::HashMap;

use super::{event_bus_field_type::EventFieldType, event_type::EventType};

const MIN_MESSAGE_LENGTH: usize = 2;

#[derive(Clone)]
pub struct EventBusMessage {
    title: String,
    fields: HashMap<EventFieldType, String>,
    event_type: EventType,
    timestamp: i64,
}

impl EventBusMessage {
    pub fn new(
        title: &str,
        event_type: EventType,
        fields: Option<Vec<(EventFieldType, &str)>>,
    ) -> Self {
        let fields = match fields {
            Some(res) => EventBusMessage::create_fields(res),
            None => None,
        };

        Self {
            title: title.to_string(),
            fields: fields.unwrap_or(HashMap::new()),
            event_type,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        let binding = String::from_utf8(data).expect("unable to decode data");
        let mut decoded = binding.split(',').collect::<Vec<&str>>();

        if decoded.len() < MIN_MESSAGE_LENGTH {
            panic!("invalid message, got {binding}");
        }

        let event_type = match decoded[1].parse::<u8>() {
            Ok(res) => res,
            Err(_) => panic!("unable to parse EventType, got {}", decoded[1]),
        };

        let fields = decoded.split_off(2).iter().fold(
            HashMap::new(),
            |mut acc: HashMap<EventFieldType, String>, elem| {
                let split = elem
                    .split('=')
                    .map(|elem| elem.to_string())
                    .collect::<Vec<String>>();
                acc.insert(EventFieldType::from_string(&split[0]), split[1].clone());
                acc
            },
        );

        Self {
            title: String::from(decoded[0]),
            event_type: EventType::from_u8(event_type),
            fields,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn format(&self) -> String {
        let formatted_fields =
            self.fields
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (i, (k, v))| {
                    format!(
                        "{acc}{}={v}{}",
                        k.to_string(),
                        if i != self.fields.len() - 1 { "," } else { "" }
                    )
                });
        format!("{},{},{}", self.title, self.event_type, formatted_fields)
    }

    pub fn format_bytes(&self) -> Vec<u8> {
        self.format().into_bytes().to_vec()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn get_field(&self, key: EventFieldType) -> String {
        self.fields
            .get(&key)
            .unwrap_or(&format!("'{}' not found", key.to_string()))
            .clone()
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn ts(&self) -> i64 {
        self.timestamp
    }

    fn create_fields(
        fields: Vec<(EventFieldType, &str)>,
    ) -> Option<HashMap<EventFieldType, String>> {
        Some(
            fields
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect(),
        )
    }
}
