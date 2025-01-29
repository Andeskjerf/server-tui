use std::collections::HashMap;

use super::{event_bus_field_type::EventFieldType, event_type::EventType};

const MIN_MESSAGE_LENGTH: usize = 2;

#[derive(Clone, Debug)]
pub struct EventBusMessage {
    title: String,
    fields: HashMap<EventFieldType, Vec<u8>>,
    event_type: EventType,
    timestamp: i64,
}

impl EventBusMessage {
    pub fn new(
        title: &str,
        event_type: EventType,
        fields: Option<Vec<(EventFieldType, Vec<u8>)>>,
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
        let mut decoded = binding.split('|').collect::<Vec<&str>>();

        if decoded.len() < MIN_MESSAGE_LENGTH {
            panic!("invalid message, got {binding}");
        }

        let event_type = match decoded[1].parse::<u8>() {
            Ok(res) => EventType::from_u8(res),
            Err(_) => panic!("unable to parse EventType, got {}", decoded[1]),
        };

        let fields = decoded.split_off(2).iter().fold(
            HashMap::new(),
            |mut acc: HashMap<EventFieldType, Vec<u8>>, elem| {
                let split = elem
                    .replace(" ", "")
                    .split('=')
                    .map(|elem| elem.to_string())
                    .collect::<Vec<String>>();

                let field_type = EventFieldType::from_string(&split[0]);
                acc.insert(
                    field_type.clone(),
                    EventBusMessage::string_to_bytes(field_type, split[1].clone()),
                );
                acc
            },
        );

        Self {
            title: String::from(decoded[0]),
            event_type,
            fields,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn format(&self) -> String {
        let formatted_fields = self
            .fields
            .iter()
            .enumerate()
            .fold(vec![], |acc, (i, (k, v))| {
                format!(
                    "{}{}={:?}{}",
                    String::from_utf8(acc).unwrap(),
                    k.to_string(),
                    v,
                    if i != self.fields.len() - 1 { "|" } else { "" }
                )
                .into_bytes()
            });
        format!(
            "{}|{}|{}",
            self.title,
            self.event_type,
            String::from_utf8(formatted_fields).unwrap()
        )
    }

    pub fn format_bytes(&self) -> Vec<u8> {
        self.format().into_bytes().to_vec()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn get_field(&self, key: EventFieldType) -> Vec<u8> {
        self.fields
            .get(&key)
            .unwrap_or(&format!("'{}' not found", key.to_string()).into_bytes())
            .clone()
    }

    pub fn get_field_string(&self, key: EventFieldType) -> String {
        String::from_utf8(self.get_field(key)).unwrap()
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn ts(&self) -> i64 {
        self.timestamp
    }

    fn create_fields(
        fields: Vec<(EventFieldType, Vec<u8>)>,
    ) -> Option<HashMap<EventFieldType, Vec<u8>>> {
        Some(fields.into_iter().collect())
    }

    fn string_to_bytes(field_type: EventFieldType, input: String) -> Vec<u8> {
        match field_type {
            EventFieldType::Description => input.into_bytes(),
            EventFieldType::Memory | EventFieldType::Cpu => input
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .filter_map(|s| s.trim().parse::<u8>().ok())
                .collect(),
        }
    }
}
