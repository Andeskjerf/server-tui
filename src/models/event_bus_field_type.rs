#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EventFieldType {
    Description,
}

impl EventFieldType {
    pub fn from_string(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "description" => EventFieldType::Description,
            &_ => panic!("invalid field type, got {input}"),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            EventFieldType::Description => "description",
        }
    }
}
