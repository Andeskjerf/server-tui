#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EventFieldType {
    Description,
    Memory,
    Cpu,
    Timestamp,
}

impl EventFieldType {
    pub fn from_string(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "description" => EventFieldType::Description,
            "memory" => EventFieldType::Memory,
            "cpu" => EventFieldType::Cpu,
            "timestamp" => EventFieldType::Timestamp,
            &_ => panic!("invalid field type, got {input}"),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            EventFieldType::Description => "description",
            EventFieldType::Memory => "memory",
            EventFieldType::Cpu => "cpu",
            EventFieldType::Timestamp => "timestamp",
        }
    }
}
