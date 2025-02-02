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
use log::trace;
use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};

use super::event_bus::EventBus;

pub const EVENT_TOPIC: &str = "process_watcher";

pub struct ProcessWatcher {
    system: Arc<Mutex<System>>,
    to_watch: Arc<Mutex<Vec<String>>>,
    event_bus: Arc<Mutex<EventBus>>,
}

impl ProcessWatcher {
    pub fn new(event_bus: Arc<Mutex<EventBus>>, processes_to_watch: Vec<String>) -> Self {
        Self {
            system: Arc::new(Mutex::new(System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
            ))),
            event_bus,
            to_watch: Arc::new(Mutex::new(processes_to_watch)),
        }
    }

    fn watch_processes(
        event_bus: Arc<Mutex<EventBus>>,
        system: Arc<Mutex<System>>,
        to_watch: Arc<Mutex<Vec<String>>>,
    ) {
        let mut lock = system.lock().unwrap();
        (*lock).refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cmd(UpdateKind::Always),
        );
        let mut count_found = 0;
        for process in (*lock).processes().values() {
            let watch_lock = to_watch.lock().unwrap();
            if count_found >= watch_lock.len() {
                break;
            }

            let name = process.name().to_str().unwrap().to_lowercase();
            let pos = watch_lock
                .iter()
                .position(|elem| name.contains(&elem.to_lowercase()));

            let bytes = "Running".as_bytes().to_vec();
            if pos.is_some() {
                trace!("before send: {:?}", bytes);
                let pos = pos.unwrap();
                event_bus.lock().unwrap().publish(
                    EVENT_TOPIC,
                    EventBusMessage::new(
                        watch_lock.get(pos).unwrap(),
                        EventType::Process,
                        Some(vec![(EventFieldType::Description, bytes)]),
                    )
                    .format_bytes(),
                );
                count_found += 1;
            }
        }
    }
}

impl Runnable for ProcessWatcher {
    fn run(&self) {
        let event_bus = Arc::clone(&self.event_bus);
        let system = Arc::clone(&self.system);
        let to_watch = Arc::clone(&self.to_watch);

        tokio::spawn(async move {
            loop {
                ProcessWatcher::watch_processes(
                    Arc::clone(&event_bus),
                    Arc::clone(&system),
                    Arc::clone(&to_watch),
                );
                sleep(Duration::from_millis(1000));
            }
        });
    }
}
