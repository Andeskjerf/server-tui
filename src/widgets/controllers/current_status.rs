use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
    thread::sleep,
    time::Duration,
};

use crate::services::{event_bus::EventBus, process_watcher};

pub type ActiveMessages = HashMap<String, (String, i64)>;
pub type Messages = Arc<Mutex<ActiveMessages>>;

const DEFAULT_STATUS_TITLE: &str = "All good!";
const DEFAULT_STATUS_DESC: &str = "Nothing happening";
const STATUS_DEFAULT_DESC: &str = "Running";

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
        event_bus
            .lock()
            .unwrap()
            .subscribe(process_watcher::EVENT_TOPIC, move |data| {
                CurrentStatusController::on_event(Arc::clone(&active_messages), data);
            });
    }

    fn cleanup_task(active_messages: Messages) {
        tokio::spawn(async move {
            loop {
                let mut lock = active_messages.lock().unwrap();
                let now = chrono::Utc::now().timestamp();
                (*lock).clone().into_iter().for_each(|(key, (_, ts))| {
                    if key == DEFAULT_STATUS_TITLE {
                        return;
                    }

                    if now - ts >= CLEANUP_INTERVAL as i64 {
                        (*lock).remove(&key);
                    }
                });
                if lock.is_empty() {
                    lock.insert(
                        DEFAULT_STATUS_TITLE.to_string(),
                        (DEFAULT_STATUS_DESC.to_string(), 0),
                    );
                }
                drop(lock);
                sleep(Duration::from_millis(100));
            }
        });
    }

    fn on_event(active_messages: Messages, data: Vec<u8>) {
        let decoded = String::from_utf8(data).expect("unable to decode data");
        let mut lock = active_messages.lock().unwrap();
        if lock.len() == 1 && lock.get(DEFAULT_STATUS_TITLE).is_some() {
            lock.remove(DEFAULT_STATUS_TITLE);
        }
        lock.insert(
            decoded,
            (
                STATUS_DEFAULT_DESC.to_string(),
                chrono::Utc::now().timestamp(),
            ),
        );
    }

    pub fn get_message_lock(&self) -> MutexGuard<ActiveMessages> {
        self.active_messages.lock().unwrap()
    }
}
