use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{
    models::{
        event_bus_field_type::EventFieldType, event_bus_message::EventBusMessage,
        event_type::EventType,
    },
    traits::runnable::Runnable,
};

pub const EVENT_TOPIC: &str = "datetime_timestamp";

use super::event_bus::EventBus;

pub struct DateTimeService {
    event_bus: Arc<Mutex<EventBus>>,
}

impl DateTimeService {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        Self { event_bus }
    }

    fn poll(event_bus: Arc<Mutex<EventBus>>) {
        let ts = chrono::Utc::now().timestamp().to_le_bytes().to_vec();
        event_bus.lock().unwrap().publish(
            EVENT_TOPIC,
            EventBusMessage::new(
                "timestamp",
                EventType::Timestamp,
                Some(vec![(EventFieldType::Timestamp, ts)]),
            )
            .format_bytes(),
        )
    }
}

impl Runnable for DateTimeService {
    fn run(&self) {
        let event_bus = Arc::clone(&self.event_bus);

        tokio::spawn(async move {
            loop {
                DateTimeService::poll(Arc::clone(&event_bus));
                sleep(Duration::from_millis(1000));
            }
        });
    }
}
