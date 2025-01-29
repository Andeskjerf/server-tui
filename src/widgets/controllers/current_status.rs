use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
    thread::sleep,
    time::Duration,
};

use crate::{
    models::{
        event_bus_field_type::EventFieldType, event_bus_message::EventBusMessage,
        event_type::EventType,
    },
    services::{event_bus::EventBus, process_watcher, socket},
};

pub type ActiveMessages = HashMap<String, EventBusMessage>;
pub type Messages = Arc<Mutex<ActiveMessages>>;

const DEFAULT_STATUS_TITLE: &str = "All good!";
const DEFAULT_STATUS_DESC: &str = "Nothing happening";

const CLEANUP_INTERVAL: u8 = 2;

pub struct CurrentStatusController {
    pub active_messages: Messages,
}

impl CurrentStatusController {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        let active_messages = Arc::new(Mutex::new(HashMap::new()));
        CurrentStatusController::cleanup_task(Arc::clone(&active_messages));
        CurrentStatusController::subscribe(Arc::clone(&event_bus), Arc::clone(&active_messages));
        Self { active_messages }
    }

    fn subscribe(event_bus: Arc<Mutex<EventBus>>, active_messages: Messages) {
        let mut lock = event_bus.lock().unwrap();
        let active_messages_1 = Arc::clone(&active_messages);
        let active_messages_2 = Arc::clone(&active_messages);

        // watch processes
        lock.subscribe(process_watcher::EVENT_TOPIC, move |data| {
            CurrentStatusController::on_event(Arc::clone(&active_messages_1), data);
        });

        // watch messages on socket
        lock.subscribe(socket::EVENT_TOPIC, move |data| {
            CurrentStatusController::on_event(Arc::clone(&active_messages_2), data);
        });
    }

    fn cleanup_task(active_messages: Messages) {
        tokio::spawn(async move {
            loop {
                let mut lock = active_messages.lock().unwrap();
                let now = chrono::Utc::now().timestamp();
                (*lock).clone().into_iter().for_each(|(key, msg)| {
                    // don't delete socket messages, instead wait for a explicit message
                    // we also don't want to remove the default status message
                    if key == DEFAULT_STATUS_TITLE || *msg.event_type() == EventType::Socket {
                        return;
                    }

                    if now - msg.ts() >= CLEANUP_INTERVAL as i64 {
                        (*lock).remove(&key);
                    }
                });
                if lock.is_empty() {
                    lock.insert(
                        DEFAULT_STATUS_TITLE.to_string(),
                        EventBusMessage::new(
                            DEFAULT_STATUS_TITLE,
                            EventType::Process,
                            Some(vec![(
                                EventFieldType::Description,
                                DEFAULT_STATUS_DESC.as_bytes().to_vec(),
                            )]),
                        ),
                    );
                }
                drop(lock);
                sleep(Duration::from_millis(100));
            }
        });
    }

    fn on_event(active_messages: Messages, data: Vec<u8>) {
        let msg = EventBusMessage::from_bytes(data);

        let mut lock = active_messages.lock().unwrap();
        if lock.len() == 1 && lock.get(DEFAULT_STATUS_TITLE).is_some() {
            lock.remove(DEFAULT_STATUS_TITLE);
        }

        // remove if the message says it's SOCKET_DONE_TEXT and it's a socket
        if *msg.event_type() == EventType::Socket
            && msg
                .get_field_string(EventFieldType::Description)
                .to_lowercase()
                == socket::SOCKET_DONE_TEXT
            && lock.contains_key(msg.title())
        {
            lock.remove(msg.title());
            return;
        }

        lock.insert(msg.title().to_string(), msg);
    }

    pub fn get_message_lock(&self) -> MutexGuard<ActiveMessages> {
        self.active_messages.lock().unwrap()
    }
}
