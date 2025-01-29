use std::sync::{Arc, Mutex};

use chrono::{TimeZone, Utc};
use log::trace;

use crate::{
    models::{event_bus_field_type::EventFieldType, event_bus_message::EventBusMessage},
    services::{datetime, event_bus::EventBus},
    utils::bytes_helper::bytes_to_i64,
};

pub struct DateTimeController {
    timestamp: Arc<Mutex<i64>>,
}

impl DateTimeController {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        let timestamp = Arc::new(Mutex::new(chrono::Utc::now().timestamp()));
        DateTimeController::subscribe(Arc::clone(&event_bus), Arc::clone(&timestamp));
        Self { timestamp }
    }

    fn subscribe(event_bus: Arc<Mutex<EventBus>>, timestamp: Arc<Mutex<i64>>) {
        let mut lock = event_bus.lock().unwrap();
        lock.subscribe(datetime::EVENT_TOPIC, move |data| {
            DateTimeController::on_event(data, Arc::clone(&timestamp));
        });
    }

    fn on_event(data: Vec<u8>, timestamp: Arc<Mutex<i64>>) {
        let msg = EventBusMessage::from_bytes(data);
        trace!("DateTimeController: on_event: {:?}", msg);

        let ts = bytes_to_i64(msg.get_field(EventFieldType::Timestamp));
        (*timestamp.lock().unwrap()) = ts;
    }

    pub fn get_formatted_time(&self) -> String {
        let ts = self.timestamp.lock().unwrap();
        let datetime = Utc.timestamp_opt(*ts, 0);
        datetime.unwrap().format("%H:%M:%S").to_string()
    }
}
