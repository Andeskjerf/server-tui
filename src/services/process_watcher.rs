use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{traits::runnable::Runnable, widgets::controllers::current_status};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use super::event_bus::EventBus;

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
        (*lock).refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        for elem in to_watch.lock().unwrap().iter() {
            for process in (*lock).processes().values() {
                let name = process.name().to_str().unwrap().to_lowercase();
                if name.contains(&elem.to_lowercase()) {
                    event_bus
                        .lock()
                        .unwrap()
                        .publish(current_status::EVENT_TOPIC, elem.as_bytes().to_vec());
                    break;
                }
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
                sleep(Duration::from_millis(100));
            }
        });
    }
}
